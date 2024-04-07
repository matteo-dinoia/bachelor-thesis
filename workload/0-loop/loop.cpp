#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <unistd.h>
#include <sys/time.h>
#include <sched.h>

void handle_signal(int sig) {
	if (sig == SIGALRM)
		exit(0);
}

// Possible wait to test
// sudo trace-cmd record -e sched -F ./loop.out

int main() {
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

	// Registering signal handler
	sa.sa_handler = &handle_signal;
	sigemptyset(&sa.sa_mask);
	sa.sa_flags = 0;
	if (sigaction(SIGALRM, &sa, NULL) == -1) {
		perror("sigaction");
		return 1;
	}

	// Setting the timer
	timer.it_value.tv_sec = 0;
	timer.it_value.tv_usec = 200 * 1000;
	timer.it_interval.tv_sec = 0;
	timer.it_interval.tv_usec = 0;

	if (setitimer(ITIMER_REAL, &timer, NULL) == -1) {
		perror("setitimer");
		return 1;
	}

	// Busy waiting to keep the program alive
	int counter = 0;
	while (1)
		counter++;

	return 0;
}
