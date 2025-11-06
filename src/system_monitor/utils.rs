
/*

Piu + alto meglio è
let d = health_cal(0.33,0.33,0.33);
*/

pub fn health_cal(wcpu: f64, wram: f64, wstorage: f64) -> f64 {

    /* CPU */
    let cpu = cpu_check();
    
    let mut avg_core: f64 = 0.0;
    let mut count_core: f64 = 0.0;
    for core in &cpu.cores {
        avg_core += core.usage as f64; 
        count_core += 1.0;
    }
    
    avg_core = (avg_core / count_core) ;
    avg_core /=  100.00;


    /*RAM*/

    let ram_s = ram_check();
    let mut avg_ram: f64 = (ram_s.gb_used as f64 / ram_s.gb_tot as f64) * 100.00;
    avg_ram /= 100.00;

    /* Disk */
        let d= storage_check();
    let mut ts: f64 = 0.0;
    let mut avs: f64 = 0.0;
    let mut count = 0;
    for v in d {
       ts += v.total_space;
       avs += v.available_space;
       count += 1;
        
    }    

    ts /= count as f64;
    avs /= count as f64;

    let mut avg_store = (avs / ts) * 100.00;
    avg_store /= 100.00;
    avg_store = 1.00 - avg_store;


    let health: f64 = wcpu * (1.0 - avg_core) + wram * (1.0 - avg_ram) + wstorage * ( 1.0 - avg_store) ; 
    
    health
}