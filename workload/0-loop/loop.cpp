#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif

#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <unistd.h>
#include <sys/time.h>
#include <sched.h>


volatile bool running;
volatile long long int first_half_work = -1;
volatile long long int counter = 0;

void handle_signal(int sig);


void setup_shed_param(){
	// Setting itself in the sched_ext class (0 = itself, 7 = sched_ext)
	struct sched_param par;
	sched_getparam(0,  &par);
	int res = sched_setscheduler(0, 7, &par);
	if (res != 0) {
		perror("sched_setscheduler");
		exit(1);
	}

	// Setting itself on last cpu (15)
	cpu_set_t mask;
	CPU_ZERO(&mask);
	CPU_SET(15, &mask);
	if (sched_setaffinity(0, sizeof(mask), &mask) == -1) {
		perror("cpuaffinity");
		exit(1);
	}
}

void start_timer(int  number_millisec){
	struct sigaction sa;
	struct itimerval timer;

	// Registering signal handler
	sa.sa_handler = &handle_signal;
	sigemptyset(&sa.sa_mask);
	sa.sa_flags = 0;
	const int res1 = sigaction(SIGALRM, &sa, NULL);
	const int res2 = sigaction(SIGUSR1, &sa, NULL);
	if (res1 == -1 || res2 == -1) {
		perror("sigaction");
		exit(1);
	}

	// Setup timer
	int number_sec = number_millisec / 1000;
	number_millisec = number_millisec % 1000;
	timer.it_value.tv_sec = number_sec;
	timer.it_value.tv_usec = number_millisec * 1000L;
	timer.it_interval.tv_sec = 0;
	timer.it_interval.tv_usec = 0;

	if (setitimer(ITIMER_REAL, &timer, NULL) == -1) {
		perror("setitimer");
		exit(1);
	}
}

int get_ms(int argc, char **argv){
	// Get timer value
	if (argc != 2) {
		printf("Usage: %s <number_millisec>\n", argv[0]);
		return -1;
	}

	// Convert the string argument to a number
	int number_millisec = atoi(argv[1]);

	// Check if conversion was successful
	if (number_millisec <= 0 && argv[1][0] != '0') {
		printf("Invalid number: %s\n", argv[1]);
		return -1;
	}

	return number_millisec;
}

void handle_signal(int sig) {
	if(sig == SIGUSR1){
		first_half_work = counter;
	}else if (sig == SIGALRM){
		running = false;
	}
}

int main(int argc, char **argv) {
	int ms = get_ms(argc, argv);
	if(ms == -1)
		return -1;

	setup_shed_param();
	start_timer(ms);
	running = true;

	// Busy waiting to keep the program alive
	while (running)
		counter++;

	// Print result
	printf("File %s made: %lld", argv[0], counter);
	if(first_half_work != -1){
		printf(" (%lld before and %lld after)",
			   first_half_work, (counter - first_half_work));
	}
	printf("\n");



	return 0;
}
