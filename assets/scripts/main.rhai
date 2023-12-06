
fn update(){
    let speed = 1;
    let d = get_dir();
    add_pos(d[0]*speed, d[1]*speed);
    if(get_key_down("p")){
        set_pos(200,200);
    }
}