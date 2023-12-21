
fn update(){
    let speed = 3;
    if(buffer[0] == 0){
        buffer[0] = speed;
        //buffer[1] = speed;
    }
    let pos = get_pos();
    if(abs(pos[0]) > 450){
        set_pos(0, 0);
        buffer[0] = speed;
        buffer[1] = 0;
    }
    if(collision_start()){
        if(abs(pos[1]) >= 260){
            buffer[1] = -buffer[1];
        }

        if abs(pos[0]) >= 360{

            if abs(pos[0]+5) < abs(statik_buffer[1]){
                buffer[0] *= -1;
            }

            if(pos[1]>statik_buffer[0]){
                buffer[1] = speed;
            }
            if(pos[1]<statik_buffer[0]){
               buffer[1] = -speed;
           }

        }
    }
    add_pos(buffer[0], buffer[1]);
    save_buff(buffer);
}