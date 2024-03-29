use std::{io::{self, Write}, mem::MaybeUninit};

use libc::{sched_getscheduler, sched_setscheduler, sched_getparam, sched_param};


/*extern "C" {
    fn sched_getscheduler(pid: c_int) -> c_int;
}*/

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
    let mut number_str = String::new();

    loop{
        print!("{}", query);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut number_str).unwrap();

        if let Ok(pid) = number_str.trim().parse::<i32>(){
            return pid;
        }
    }
    
    
}

fn set_method(){
    let pid = read_number("Insert pid on which to write: ");
    let policy = read_number("Insert policy to change to: ");
    let mut par = MaybeUninit::<sched_param>::uninit();

    unsafe { sched_getparam(pid,  par.as_mut_ptr()) };

    let res = unsafe{sched_setscheduler(pid, policy, par.as_ptr())};
    if res == 0 {
        println!("Successfully changed to {}", get_priority(policy));
    }else{
        println!("Failed to change to {}", get_priority(policy));
    }
}

fn get_method(){
    let pid = read_number("Insert pid on which to read: ");
    
    let policy = unsafe { sched_getscheduler(pid) };
    println!("Scheduler policy for process {}: {} ({})", pid, get_priority(policy), policy);
}

fn main() {
    let mut command = String::new();
    
    loop{
        print!("\nInsert what to do (read/write/exit [R/w/any]): ");
        io::stdout().flush().unwrap();
        
        command.clear();
        io::stdin().read_line(&mut command).unwrap();

        match command.trim().to_lowercase().as_str() {
            "r" => get_method(),
            "" => get_method(),
            "w" => set_method(),
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

// TODO i need to add the option to a scheduler and then to decide to move pid there
