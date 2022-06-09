use std::thread;

//#ifndef _N
//#define _N 5
//#endif
const _N: usize = 5;
// 
//enum{
//	pensando = 0,
//	comiendo,
//};
// 
//std::binary_semaphore tenedores[_N] = {
//	std::binary_semaphore(1),
//	std::binary_semaphore(1),
//	std::binary_semaphore(1),
//	std::binary_semaphore(1),
//	std::binary_semaphore(1),
//};

fn filosofo(fil_num : usize, &sim : &bool, &estoy : &i32){
//	int intervalo;
//	std::cout<<"Filosofo "<<fil_num<<" inicia:"<<std::endl;
//	while (*sim) {
//		intervalo = rand()%10;
//		sleep(intervalo);
//		if(*estoy==pensando){
//			tenedores[(fil_num+(fil_num%2))%_N].acquire(); 
//			tenedores[((fil_num+1)-(fil_num%2))%_N].acquire(); 
//			*estoy = comiendo;
//		} else {
//			tenedores[fil_num].release();
//			tenedores[(fil_num+1)%_N].release();
//			*estoy = pensando;
//		}
//	}
//	if(*estoy==comiendo){
//		tenedores[fil_num].release();
//		tenedores[(fil_num+1)%_N].release();
//	}
//	std::cout<<"Filosofo "<<fil_num<<" termina"<<std::endl;
}

fn main(){
	let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
	let filosofos: [i32;_N] = [0;_N];
	let sim : bool = true;
	let estado: [&str;2] = [
		"pensando",
		"comiendo"
	];
	
	for i in 0.._N {
		threads.push(thread::spawn(move || { filosofo(i, &sim, &filosofos[i]) }));
	}
}

//	std::cout<<"Inicio:\n\n";
//	for(i=0;i<60;i++){
//		for(int j=0;j<5;j++){
//			std::cout<<"Filosofo "<<j<<", está: "<<estado[filosofos[j]]<<std::endl;
//		}
//		std::cout<<"\n";
//		sleep(1);
//	}
//   
//	sim = 0;
//	
//	for(i=0;i<_N;i++){
//		threads[i].wait();
//	}
//	
//	std::cout<<"\nFIN";
//
////	return 0;
//}
