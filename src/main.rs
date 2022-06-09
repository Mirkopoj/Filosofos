#include <iostream>
#include <future>
#include <semaphore>
#include <unistd.h>

#ifndef _N
#define _N 5
#endif

enum{
	pensando = 0,
	comiendo,
};

std::binary_semaphore tenedores[_N] = {
	std::binary_semaphore(1),
	std::binary_semaphore(1),
	std::binary_semaphore(1),
	std::binary_semaphore(1),
	std::binary_semaphore(1),
};

void filosofo(const int fil_num, const char *sim, int *estoy){
	int intervalo;
	std::cout<<"Filosofo "<<fil_num<<" inicia:"<<std::endl;
	while (*sim) {
		intervalo = rand()%10;
		sleep(intervalo);
		if(*estoy==pensando){
			tenedores[(fil_num+(fil_num%2))%_N].acquire(); 
			tenedores[((fil_num+1)-(fil_num%2))%_N].acquire(); 
			*estoy = comiendo;
		} else {
			tenedores[fil_num].release();
			tenedores[(fil_num+1)%_N].release();
			*estoy = pensando;
		}
	}
	if(*estoy==comiendo){
		tenedores[fil_num].release();
		tenedores[(fil_num+1)%_N].release();
	}
	std::cout<<"Filosofo "<<fil_num<<" termina"<<std::endl;
}

int main(){
	std::future<void> threads[_N];
	int filosofos[_N] = {0};
	char sim = 1;
	const char* estado[2] = {
		"pensando",
		"comiendo"
	};
	
	int i;

	for(i=0;i<_N;i++){
		threads[i] = std::async(std::launch::async, filosofo, i, &sim, &filosofos[i]);
	}

	std::cout<<"Inicio:\n\n";
	for(i=0;i<60;i++){
		for(int j=0;j<5;j++){
			std::cout<<"Filosofo "<<j<<", está: "<<estado[filosofos[j]]<<std::endl;
		}
		std::cout<<"\n";
		sleep(1);
	}
   
	sim = 0;
	
	for(i=0;i<_N;i++){
		threads[i].wait();
	}
	
	std::cout<<"\nFIN";

	return 0;
}
