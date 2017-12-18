use std::thread;

mod instructions;

#[cfg(not(test))]
fn main() {
    const INPUT: &str = include_str!("input.txt");
    let program = instructions::parse(INPUT);

    println!("Part 1: {}", find_first_recovered(&program));
    println!("Part 2: {}", run_threads(program));
}

/// Updates a `register` using it's current value and the given `value` using the function `f`
fn update_register<T>(
    registers: &mut instructions::Registers,
    register: instructions::RegisterName, value: &instructions::Value,
    f: T
) where T: Fn(i64, i64) -> i64
{
    let value = f(instructions::get_register(register, registers), value.get(registers));
    registers.insert(register, value);
}

/// Finds the first "recovered" value when the instructions are run
fn find_first_recovered(program: &Vec<instructions::Instruction>) -> i64 {
    use instructions::Registers;
    use instructions::Instruction::*;

    let mut program_counter = 0;
    let mut last_played = 0;
    let mut registers: Registers = Registers::new();

    loop {
        if let Some(instruction) = program.get(program_counter as usize) {
            match *instruction {
                Send(ref value)               => { last_played = value.get(&registers) },
                Receive(register)             => if instructions::get_register(register, &registers) != 0 { break; },
                Set(register, ref value)      => update_register(&mut registers, register, value, | _, v | v),
                Add(register, ref value)      => update_register(&mut registers, register, value, | r, v | r + v),
                Multiply(register, ref value) => update_register(&mut registers, register, value, | r, v | r * v),
                Modulus(register, ref value)  => update_register(&mut registers, register, value, | r, v | r % v),
                Jump(ref x, ref y)            => if x.get(&registers) > 0 { program_counter = program_counter + y.get(&registers) - 1; },
            }

            program_counter = program_counter + 1;
        } else {
            panic!("No instruction found at PC {}", program_counter);
        }
    }

    last_played
}

/// Runs two programs which can communicate via `Send`/`Receive` commands and when they deadlock
/// returns the number of times the second program used the `Send` instruction
fn run_threads(program: Vec<instructions::Instruction>) -> i32 {
    use instructions::Registers;
    use instructions::Instruction::*;
    use std::sync::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Create the channels
    let lock_count = Arc::new(AtomicUsize::new(0));
    let channels: [Vec<i64>; 2] = [vec![], vec![]];
    let channels = Arc::new(Mutex::new(channels));

    // Create two "programs"
    let mut threads: Vec<_> = (0 .. 2).map( | program_number: usize | {
        // Copy the data we need inside the thread before we move it there
        let program = program.clone();
        let channels = channels.clone();
        let lock_count = lock_count.clone();
        let other_program: usize = (program_number + 1) % 2;

        // Create the thread
        thread::spawn( move || {
            let mut program_counter = 0;
            let mut registers: Registers = Registers::new();
            let mut send_counter = 0;
            let mut deadlock_found = false;
            registers.insert('p', program_number as i64);

            // Keep looping the program instructions
            while !deadlock_found {
                if let Some(instruction) = program.get(program_counter as usize) {
                    match *instruction {
                        Send(ref value)               => {
                            send_counter = send_counter + 1;
                            let value = value.get(&registers);

                            let mut other = channels.lock().unwrap();
                            other[other_program].push(value);
                        },
                        Receive(register)             => {
                            let mut is_already_waiting = false;

                            // Loop until a value comes or both threads a waiting
                            loop {
                                let mut channels = channels.lock().unwrap();
                                let channel = &mut channels[program_number];
                                if channel.is_empty() {
                                    // Get the number of threads waiting
                                    let wait_count = if !is_already_waiting {
                                        is_already_waiting = true;
                                        lock_count.fetch_add(1, Ordering::SeqCst)
                                    } else {
                                        lock_count.load(Ordering::SeqCst)
                                    };

                                    // If all threads are waiting, deadlock found
                                    if wait_count == 2 {
                                        deadlock_found = true;
                                        break;
                                    }
                                } else {
                                    let value = channel.remove(0);
                                    if is_already_waiting {
                                        lock_count.fetch_sub(1, Ordering::SeqCst);
                                    }

                                    registers.insert(register, value);
                                    break;
                                }
                            }
                        },

                        // These are the same as before
                        Set(register, ref value)      => update_register(&mut registers, register, value, | _, v | v),
                        Add(register, ref value)      => update_register(&mut registers, register, value, | r, v | r + v),
                        Multiply(register, ref value) => update_register(&mut registers, register, value, | r, v | r * v),
                        Modulus(register, ref value)  => update_register(&mut registers, register, value, | r, v | r % v),
                        Jump(ref x, ref y)            => if x.get(&registers) > 0 { program_counter = program_counter + y.get(&registers) - 1; },
                    }

                    program_counter = program_counter + 1;
                } else {
                    panic!("No instruction found at PC {}", program_counter);
                }
            }

            send_counter
        })
    }).collect();

    // Wait for program 1 to quit and sell
    threads.remove(1).join().expect("Program 1 Did not return a value")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "
        set a 1
        add a 2
        mul a a
        mod a 5
        snd a
        set a 0
        rcv a
        jgz a -1
        set a 1
        jgz a -2
    ";

    #[test]
    fn test_parse() {
        use instructions::Instruction::*;
        use instructions::Value::*;

        let instructions = instructions::parse(INPUT);

        assert_eq!(
            vec![
                Set('a', Value(1)),
                Add('a', Value(2)),
                Multiply('a', Register('a')),
                Modulus('a', Value(5)),
                Send(Register('a')),
                Set('a', Value(0)),
                Receive('a'),
                Jump(Register('a'), Value(-1)),
                Set('a', Value(1)),
                Jump(Register('a'), Value(-2)),
            ],
            instructions
        );
    }

    #[test]
    fn test_first_recovered() {
        assert_eq!(
            find_first_recovered(&instructions::parse(INPUT)),
            4
        );
    }

    #[test]
    fn test_threads() {
        let program = instructions::parse("
            snd 1
            snd 2
            snd p
            rcv a
            rcv b
            rcv c
            rcv d
        ");

        assert_eq!(
            run_threads(program),
            3
        );
    }
}
