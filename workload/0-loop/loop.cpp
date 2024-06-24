#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <unistd.h>
#include <sys/time.h>
#include <sched.h>

#define ASSERT(cond, err_str) \
	if((cond)){ perror((err_str)); exit(1); }

volatile bool running;
volatile long long int first_half_work = -1;
volatile long long int counter = 0;

void handle_signal(int sig);


void setup_shed_param(){
	// Setting itself (0) in the sched_ext class (7)
	struct sched_param par;
	sched_getparam(0,  &par);
	const int ret1 = sched_setscheduler(0, 7, &par);
	ASSERT(ret1 != -1, "sched_setscheduler");

	// Setting itself on last cpu (15)
	cpu_set_t mask;
	CPU_ZERO(&mask);
	CPU_SET(15, &mask);
	const int ret2 = sched_setaffinity(0, sizeof(mask), &mask);
	ASSERT(ret2 != -1, "cpuaffinity");
}

void start_timer(int  number_millisec){
	struct sigaction sa;
	struct itimerval timer;

	// Registering signal handler
	sa.sa_handler = &handle_signal;
	sigemptyset(&sa.sa_mask);
	sa.sa_flags = 0;
	const int ret1 = sigaction(SIGALRM, &sa, NULL);
	const int ret2 = sigaction(SIGUSR1, &sa, NULL);
	ASSERT((ret1 == -1 || ret2 == -1), "sigaction");

	// Setup timer
	int number_sec = number_millisec / 1000;
	number_millisec = number_millisec % 1000;
	timer.it_value.tv_sec = number_sec;
	timer.it_value.tv_usec = number_millisec * 1000L;
	timer.it_interval.tv_sec = 0;
	timer.it_interval.tv_usec = 0;

	const int ret3 = setitimer(ITIMER_REAL, &timer, NULL);
	ASSERT(ret3 != -1, "cpuaffinity");
}

int get_ms(int argc, char **argv){
	// Get timer value
	ASSERT(argc != 2, "Usage: program_name <ms_duration>");

	// Convert the string argument to a number
	int millisec;
	const int ret = sscanf(argv[1], "%d", &millisec);
	ASSERT((ret == 1 && millisec > 0), 
			"First parameter should be a positive integer");

	return millisec;
}

void handle_signal(int sig) {
	if(sig == SIGUSR1)
		first_half_work = counter;
	else if (sig == SIGALRM)
		running = false;
}

int main(int argc, char **argv) {
	int ms = get_ms(argc, argv);

	setup_shed_param();
	start_timer(ms);
	running = true;

	// Busy waiting to keep the program alive
	while (running)
		counter++;

	// Print result
	printf("File %s made: %lld", argv[0], counter);
	if(first_half_work != -1){ //if was split in two times
		printf(" (%lld before and %lld after)",
			   first_half_work, (counter - first_half_work));
	}
	printf("\n");
}
