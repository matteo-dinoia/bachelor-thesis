#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <unistd.h>
#include <sys/time.h>
#include <sched.h>

volatile int test;

void handle_signal(int sig) {
	test=0;
}

int main(int argc, char **argv) {
	struct sigaction sa;
	struct itimerval timer;

	// Setting itself in the sched_ext class (0 = itself, 7 = sched_ext)
	struct sched_param par;
	sched_getparam(0,  &par);
	int res = sched_setscheduler(0, 7, &par);
	if (res != 0) {
		perror("sched_setscheduler");
		return 1;
	}

	test=1;

	// Registering signal handler
	sa.sa_handler = &handle_signal;
	sigemptyset(&sa.sa_mask);
	sa.sa_flags = 0;
	if (sigaction(SIGALRM, &sa, NULL) == -1) {
		perror("sigaction");
		return 1;
	}

	// Get timer value
	if (argc != 2) {
		printf("Usage: %s <number_millisec>\n", argv[0]);
		return 1;
	}

	// Convert the string argument to a number
	int number_millisec = atoi(argv[1]);

	// Check if conversion was successful
	if (number_millisec <= 0 && argv[1][0] != '0') {
		printf("Invalid number: %s\n", argv[1]);
		return 1;
	}

	// Setting the timer
	int number_sec = number_millisec / 1000;
	number_millisec = number_millisec % 1000;
	timer.it_value.tv_sec = number_sec;
	timer.it_value.tv_usec = number_millisec * 1000L;
	timer.it_interval.tv_sec = 0;
	timer.it_interval.tv_usec = 0;

	if (setitimer(ITIMER_REAL, &timer, NULL) == -1) {
		perror("setitimer");
		return 1;
	}

	// Busy waiting to keep the program alive
	int counter = 0;
	while (test)
		counter++;

	return 0;
}
