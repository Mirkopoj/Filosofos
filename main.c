#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>
#include <semaphore.h>

#ifndef _N
#define _N 5
#endif

typedef struct info_filosofo_struct{
	int fil_num;
	int *estoy;
	char *sim;
} info_filosofo_t;

enum{
	pensando = 0,
	comiendo,
};

sem_t tenedores[_N];
sem_t init;

void *filosofo(void *data){
	info_filosofo_t mi_info = *((info_filosofo_t *)data);
	sem_post(&init);
	int intervalo;
	printf("Filosofo %d inicia:\n", mi_info.fil_num);
	while (*(mi_info.sim)) {
		intervalo = rand()%10;
		sleep(intervalo);
		if(*(mi_info.estoy)==pensando){
			sem_wait(&tenedores[(mi_info.fil_num+(mi_info.fil_num%2))%_N]);
			sem_wait(&tenedores[((mi_info.fil_num+1)-(mi_info.fil_num%2))%_N]);
			*(mi_info.estoy) = comiendo;
		} else {
			sem_post(&tenedores[mi_info.fil_num]);
			sem_post(&tenedores[(mi_info.fil_num+1)%_N]);
			*(mi_info.estoy) = pensando;
		}
	}
	if(*(mi_info.estoy)==comiendo){
		sem_post(&tenedores[mi_info.fil_num]);
		sem_post(&tenedores[(mi_info.fil_num+1)%_N]);
	}
	printf("Filosofo %d termina\n", mi_info.fil_num);
	pthread_exit(0);
}

int main(){
	int filosofos[_N] = {0};
	pthread_t threads[_N];
	char sim = 1;
	const char* estado[2] = {
		"pensando",
		"comiendo"
	};
	
	int i;
	
	for(i=0;i<_N;i++){
		sem_init(&tenedores[i], 0, 1);
	}
	sem_init(&init, 0, 1);

	for(i=0;i<_N;i++){
		sem_wait(&init);
		info_filosofo_t data;
		data.estoy = &filosofos[i];
		data.fil_num = i;
		data.sim = &sim;
		pthread_create(&threads[i], NULL, filosofo, (void *)&data);
	}

	printf("Inicio:\n\n");
	for(i=0;i<60;i++){
		for(int j=0;j<5;j++){
			printf("Filosofo %d, está: %s\n", j, estado[filosofos[j]]);
		}
		printf("\n");
		sleep(1);
	}
   
	sim = 0;
	
	for(i=0;i<_N;i++){
		pthread_join(threads[i], NULL);
	}
	
	printf("\nFIN");

	return 0;
}
