use std::thread;
use std::time;

use rand::Rng;

const _N: usize = 5;

enum Estado{
	Pensando = 0,
	Comiendo,
}
 
//std::binary_semaphore tenedores[_N] = {
//	std::binary_semaphore(1),
//	std::binary_semaphore(1),
//	std::binary_semaphore(1),
//	std::binary_semaphore(1),
//	std::binary_semaphore(1),
//};

fn filosofo(fil_num : usize, &sim : &bool, &estoy : &usize){
	let mut intervalo: time::Duration;
    let mut rng_gen = rand::thread_rng();
	println!("El filosofo: {}, inicia.", fil_num);

    while sim {
       intervalo = time::Duration::from_secs(rng_gen.gen::<u64>()%10);
       thread::sleep(intervalo);
    }

//	while (*sim) {
//		intervalo = rand()%10;
//		sleep(intervalo);
//		if(*estoy==pensando){
//			tenedores[(fil_num+(fil_num%2))%_N].acquire(); 
//			tenedores[((fil_num+1)-(fil_num%2))%_N].acquire(); 
//			*estoy = comiendo;
//		} else {
//			tenedores[fil_num].release();
//			tenedores[(fil_num+1)%_N].release();
//			*estoy = pensando;
//		}
//	}
//	if(*estoy==comiendo){
//		tenedores[fil_num].release();
//		tenedores[(fil_num+1)%_N].release();
//	}
//	std::cout<<"Filosofo "<<fil_num<<" termina"<<std::endl;
}

fn main(){
	let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
	let filosofos: [usize;_N] = [0;_N];
	let mut sim : bool = true;
	let estado: [&str;2] = [
		"pensando",
		"comiendo"
	];
	
	for i in 0.._N {
		threads.push(thread::spawn(move || { filosofo(i, &sim, &filosofos[i]) }));
	}

    println!("Inicio:\n");

    let un_segundo = time::Duration::from_secs(1);
    for _ in 0..60 {
        for i in 0..5 {
            println!("El filosofo: {}, está: {}", i, estado[filosofos[i]]);
        }
        println!();
        thread::sleep(un_segundo);
    }

    sim = false;

    for thread in threads {
        let res = thread.join();
        match res {
            Ok(_) => continue,
            Err(e) => println!("Error: {e:?}"),
        }
    }

    println!("Fin");

}
