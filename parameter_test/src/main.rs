use std::{io::{self, Write}, mem::MaybeUninit, process::{Command, Stdio}};
use libc::{sched_getscheduler, sched_setscheduler, sched_getparam, sched_param};


fn get_priority(policy: i32) -> String{
	return match policy {
		0  => "SCHED_NORMAL",
		1  => "SCHED_FIFO",
		2  => "SCHED_RR",
		3  => "SCHED_BATCH",
		/* "SCHED_ISO": reserved but not implemented yet */
		5  => "SCHED_IDLE",
		6  => "SCHED_DEADLINE",
		7  => "SCHED_EXT",
		-1 => "PID_NOT_FOUND",
		_  => "unknown"
	}.to_string();
}

fn read_number(query : &str) -> i32{
	let mut number_str;

	loop{
		print!("{}", query);
		io::stdout().flush().unwrap();

		number_str = String::new();
		io::stdin().read_line(&mut number_str).unwrap();

		if let Ok(pid) = number_str.trim().parse::<i32>(){
			return pid;
		}
	}
	
	
}

fn set_method(pid : i32, policy : i32){
	let mut par = MaybeUninit::<sched_param>::uninit();

	unsafe { sched_getparam(pid,  par.as_mut_ptr()) };

	let res = unsafe{sched_setscheduler(pid, policy, par.as_ptr())};
	if res == 0 {
		println!("Successfully changed pid {} to {}", pid, get_priority(policy));
	}else{
		println!("Failed to change pid {} to {}", pid, get_priority(policy));
	}
}

fn get_method(pid : i32){
	let policy = unsafe { sched_getscheduler(pid) };
	println!("Scheduler policy for process {}: {} ({})", 
			pid, get_priority(policy), policy);
}

fn get_list_in_sheduling_class(sched_class : &str) -> String{
	let ps_child = Command::new("ps")
		.args(&["-Ao", "pid,class,cmd"])
		.stdout(Stdio::piped())
		.spawn()  
		.unwrap();
	let grep_child = Command::new("grep")
		.arg(sched_class)
		.stdin(Stdio::from(ps_child.stdout.unwrap())) // Pipe through.
		.stdout(Stdio::piped())
		.spawn()
		.unwrap();
	let output = grep_child.wait_with_output().unwrap();
	return String::from_utf8(output.stdout).unwrap();
}

fn main() {
	let mut command = String::new();
	
	loop{
		print!("\nInsert what to do (read/write/exit [R/w/any]): ");
		io::stdout().flush().unwrap();
		
		command.clear();
		io::stdin().read_line(&mut command).unwrap();

		match command.trim().to_lowercase().as_str() {
			"r" | "" => {
				let pid = read_number("Insert pid on which to write: ");
				get_method(pid);
			},
			"w" => {
				let pid = read_number("Insert pid on which to write: ");
				let policy = read_number("Insert policy to change to: ");
				set_method(pid, policy);
			},
			"x" => {
				let list =  get_list_in_sheduling_class("#7");
				if list.lines().count() != 0 {
					print!("Process on scheduling class ext (#7) are:\n {}", 
						list);
				} else {
					println!("No process on scheduling class ext (#7)");
				}
				
			},
			"y" => {
				println!("Resetting processes to normal classes");

				let list =  get_list_in_sheduling_class("#7");
				let array = list.lines().map(|line | {line.trim().split(" ").next()});
				array.for_each(|pid_str| {
					let pid_res = pid_str.unwrap().trim().parse::<i32>();
					if let Ok(pid) = pid_res{
						set_method(pid, 0);
					}
				});
			},
			_   => break
		};
	}
	
	println!("\nExiting because of command: {}\n", command)
}

// Look at void print_scx_info(const char *log_lvl, struct task_struct *p);
// In https://github.com/sched-ext/sched_ext/blob/sched_ext/include/linux/sched/ext.h

/* IN enum scx_ops_flags in https://github.com/sched-ext/sched_ext/blob/sched_ext/include/linux/sched/ext.h#L96
	 * If set, only tasks with policy set to SCHED_EXT are attached to
	 * sched_ext. If clear, SCHED_NORMAL tasks are also included.
*/
//SCX_OPS_SWITCH_PARTIAL	= 1LLU << 0,
