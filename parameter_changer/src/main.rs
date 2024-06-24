use std::{io::{self, Write}, mem::MaybeUninit, process::{Command, Stdio}};
use libc::{sched_getscheduler, sched_setscheduler, sched_getparam, sched_param};

fn get_priority(policy: i32) -> String{
	return match policy {
		0  => "SCHED_NORMAL",
		1  => "SCHED_FIFO",
		2  => "SCHED_RR",
		3  => "SCHED_BATCH",
		/* 4 => "SCHED_ISO": reserved but not implemented yet */
		5  => "SCHED_IDLE",
		6  => "SCHED_DEADLINE",
		7  => "SCHED_EXT",
		-1 => "PID_NOT_FOUND",
		_  => "unknown"
	}.to_string();
}

fn read_number(query : &str) -> i32{
	let mut number_str = String::new();

	loop{
		print!("{query}: ");
		io::stdout().flush().unwrap_or(());

		number_str.clear();
		io::stdin().read_line(&mut number_str).unwrap();

		if let Ok(pid) = number_str.trim().parse::<i32>(){
			return pid;
		}
	}
}

fn set_sched_class(pid : i32, policy : i32){
	let mut par = MaybeUninit::<sched_param>::uninit();

	unsafe { sched_getparam(pid,  par.as_mut_ptr()) };

	let res = unsafe{sched_setscheduler(pid, policy, par.as_ptr())};
	if res == 0 {
		println!("Successfully changed pid {} to {}", pid, get_priority(policy));
	}else{
		println!("Failed to change pid {} to {}", pid, get_priority(policy));
	}
}

fn print_sched_class(pid : i32){
	let policy = unsafe { sched_getscheduler(pid) };
	println!("Scheduler policy for process {}: {} ({})", 
			pid, get_priority(policy), policy);
}

fn get_list_str_by_sched_class(sched_class : &str) -> String{
	let ps_child = Command::new("ps")
		.args(&["-Ao", "pid,class,cmd"])
		.stdout(Stdio::piped()).spawn()  .unwrap();
	let grep_child = Command::new("grep")
		.arg(sched_class)
		.stdin(Stdio::from(ps_child.stdout.unwrap())) // Pipe through.
		.stdout(Stdio::piped()).spawn().unwrap();
	let output = grep_child.wait_with_output().unwrap();
	return String::from_utf8(output.stdout).unwrap();
}

fn main() {
	let mut command = String::new();
	
	loop{
		print!("\nInsert what to do (read/write/list_sched_ext/reset_to_norm): ");
		io::stdout().flush().unwrap_or(());
		
		command.clear();
		io::stdin().read_line(&mut command).unwrap();

		match command.trim().to_lowercase().as_str() {
			"read" => {
				let pid = read_number("Insert pid on which to write");
				print_sched_class(pid);
			},
			"write" => {
				let pid = read_number("Insert pid on which to write");
				let policy = read_number("Insert policy to change to");
				set_sched_class(pid, policy);
			},
			"list_sched_ext" => {
				let list =  get_list_str_by_sched_class("#7");
				if list.lines().count() != 0 {
					print!("Process on scheduling class ext (#7) are:\n {list}");
				} else {
					println!("No process on scheduling class ext (#7)");
				}
			},
			"reset_to_norm" => {
				println!("Resetting processes to normal classes");

				let list_str =  get_list_str_by_sched_class("#7");
				let iterator = list_str.lines().map(|line| {line.trim().split(" ").next().unwrap()});
				
				iterator.for_each(|pid_str| {
					let pid = pid_str.parse::<i32>().unwrap();
					set_sched_class(pid, 0);
				});
			},
			"h" => {
				println!();
				println!("---------------------- HELP ----------------------");
				println!("Usage:");
				println!("  r\tTo display a process scheduling class");
				println!("  w\tTo change a process scheduling class");
				println!("  l\tTo list processes in sched_ext class");
				println!("  n\tTo move all process back to sched_normal");
				println!("  q\tQuit");
				println!("--------------------------------------------------");
				println!();
			},
			_   => break
		};
	}
	
	println!("\nExiting because of command: {}\n", command)
}


