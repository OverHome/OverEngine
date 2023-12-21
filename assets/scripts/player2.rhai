

fn update(){
    let speed = 4;
    let d = get_dir_arrows();
    let p = get_pos();
    if(abs(p[1]+d[1]*speed) < 300){
         add_pos(0, d[1]*speed);
    }
    if(is_collision()){
        statik_buffer[0] = p[1];
        statik_buffer[1] = p[0];

        save_statik_buff(statik_buffer);
    }
}