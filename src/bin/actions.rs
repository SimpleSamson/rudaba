use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::time::Duration;
use async_std::task;
use databaseFx::read_data_values;
use savefile::Introspect;
//use databaseFx;
//use crate::rudaba_db::{Data, Database, DatabaseAdmins, User};
mod databaseFx;
//use memmap::MmapOptions;
fn main(){//} actions{
/// the value of a motor that determines current used to drive 
///  -ve value means bend e.g move_motor(-5, -4) -> means drive motor in anticlockwise direction bend hip -5 and bend knee -4
/// id is the unique identifier for the motor
struct Motor{
    value: i64,
    id: i64
}
union ankle_motor{
    left: std::mem::ManuallyDrop<Motor>,
    right: std::mem::ManuallyDrop<Motor>
}
pub fn get_motors(motor_name: &str) -> String{//Result<String, &'static str>{
    //TODO: create motoIDs.txt
    let mut file = File::open("MotoIDs.txt").expect("motor file expected");
//    let mut motors = String::new();
//    let motor_value = read_data_values(motorName);
//    let mut MotorValue = Vector::new;
    //let motor(motorName) = String::from(MotorValue.);
   /*  let motor_query = match file.read_at(buf, offset) {
          
    }; */
/*    let mut motors = Vec::new();
    let mut x; //:Option<&String>;// = &String::new();// Vec<_> = Vec::new();

    let mut motor_query_result:HashMap<&str, _> = HashMap::new();
    for line in motors.lines(){
        if line.contains(motor_name){
            x = motor_query_result.get(line);
        }
    }
   // let mut x = file.read_to_string(&mut motors).unwrap();
    //let mut x = motor_query_result.get();
    Ok(x);
    */
    let mut motors = String::new();
    //let mut fileContents = 
    file.read_to_string(&mut motors);
    let mut sought_motor: Vec<&str> = Vec::new();
    //let bufferx = [0; 100];
//    let mut y = fileContents.//.to_string();//.read_to_end(bufferx);
    for line in motors.lines(){
        if(line.contains(motor_name)){
            sought_motor.push(line);
        }
    }
    let mut
    let mut x  = String::from();
    return x;
}
impl Motor {

    ///  enter the motor ids that will then be saved in a file
    ///name: a string representation of the motor e.g 
    /// - ankle_motor_r : right ankle motor
    /// - ankle_motor_l : left ankle motor
    /// motorID: a string representation referencing th motor in order to pass current 
    fn set_motor_ids(name: &str, motorID: &str){

    }
    

    ///name: a string representation of the motor e.g 
    /// - ankle_motor_r : right ankle motor
    /// - ankle_motor_l : left ankle motor
    /// the return is a string value that can be used to reference the motor in order to pass current
    /// e.g 1010111
    pub fn get_motors_id(name: &str) -> &'static str{
        //let mut results = str::new();
    //    data to retrieve ids from database or file
    //  let mut motor_id_values = databaseFx::read_data(
        //   name.to_string(), 
        //   databaseFx::rudaba_db::Data{
        //       data_title: name.to_string(),
        //      ..
            // data_id: Some(_),
            // data: Some(_),
            //.. //Some(_).to_owned(),//..String,//Some(_),
            //data: Some(_)//..String//Some_
    //  });//.read_data(name, data_input);//::new();
    //    let mut valueX = "";

    //    valueX = motorIDValues.
    //return valueX;
    /*
        match name{
            ankle_motor_r{
                valueX = motor.
            }
        }
        
        ankle_motor_r: rudabaDB::read_data(
            "ankle_motor_r", 
            Data(
                data_id: .., 
                data_title: "MotorID"
            ), data: ),//i64,
        ankle_motor_l: rudabaDB::read_data("ankle_motor_l", Data(data_id: .., data_title: "MotorID"), data: ) i64,

    */

        let mut MotorID = databaseFx::read_data_values("motors".to_string());
        //from the returned data get the motor ID
        
        return &MotorID;
    }
}
/*struct Motor{
    hip: i64,
    knee: i64,

}
impl Motor{
    ///current sent to the motor
    ///hipValue in terms of motion
    /// kneeValue in terms of motion
    /// -ve value means bend e.g move_motor(-5, -4) -> means drive motor in anticlockwise direction bend hip -5 and bend knee -4
    /// +ve value = stretch 
/*    pub fn move_motor(hip_value: i64, knee_value: i64) {//-> Motor{
        let motor = Motor{
            hip: hip_value,
            knee: knee_value
        };
    } */
    pub fn move_motor(value: i64){
        let motor = Motor{value : value};
    }
//        MoveMotor{hip, knee}

} */
///limbs of the motor
impl Motor{
    ///the magnitude of the integer determines the current supplied to motor while the sign determines the direction
    /// (+) values denote a clockwise motion
    /// (-) values denote an anticlockwise motion
    

    /// leg location: left or right
    pub fn lift_leg(motion:i64, leg_location: &str){
        let knee_motor = Motor{
            value: motion, id: get_motors("knee_motor")
        };
        let hip_motion = Motor{
            value: motion * -1, id: get_motors()
        };
        let knee_motor = Motor{
            value: motion, id: get_motors_id(leg_location).parse::<i64>().unwrap()
        };
        let hip_motion = Motor{
            value: motion * -1, id: get_motors_id(leg_location).parse::<i64>().unwrap()
        };
    }

    //motion controls the speed
    //leg_location is the leg side Left or right or any alphanumeric name Use left or right if possible since it is used in moving the other ankle
    pub fn move_leg_forward(motion: i64, leg_location: &str){
        //1.lift leg up
        task::spawn(async move{
            task::sleep(Duration::from_secs(2)).await;
            let knee_motor = Motor{value: motion, id: get_motors_id(leg_location).parse::<i64>().unwrap()};
            let hip_motion = Motor{value: motion * 2 /10, id: get_motors_id(leg_location).parse::<i64>().unwrap()}; 
        });
        //2.move shin and foot forward by straightening knee while hip maintains fixed rotation
        task::spawn(async move{
            task::sleep(Duration::from_secs(2)).await;
            let knee_motor = Motor{value: motion, id: 4684684};
        });
        //3.move ankle of other foot
        task::spawn(async move{
            /*let ankle_m = ankle_motor{
                left: Motor{value: motion, id: legOther()};
            };
            task::sleep(Duration::from_secs(2)).await; */
            //choose opposite leg
            match leg_location {
                "right"=> Motor{value: motion, id: get_motors_id("left_ankle").parse::<i64>().unwrap()},// left_ankle_motor.id},
                "left" => Motor{value: motion, id: get_motors_id("right_ankle").parse::<i64>().unwrap()}, //right_ankle_motor.id}
                _ => Motor { value: motion, id: get_motors_id("left_ankle").parse::<i64>().unwrap()}
            }
          /*  if(leg.contains("right")){
                ankle_m.left = Motor{value: motion, id:}; //make ankle motor opposite calculatable
            }else if(leg.contains("left")){
                ankle_m.right = Motor{value: motion};
            } */
        });
    }

    ///leg_location: left or right
    pub fn lower_leg(motion:i64, leg_location: &str){
        let mut knee_motor = "knee_motor_left";
        let mut hip_motor ="hip_motor_right";
        match leg_location{
            "left" => { knee_motor = "knee_motor_left"; hip_motor = "hip_motor_left"},
            "right" => { knee_motor = "knee_motor_right"; hip_motor = "hip_motor_right"},
            _ => { knee_motor = "knee_motor_left"; hip_motor = "hip_motor_left"}
        }
        let knee_motor = Motor{value: motion * -1, id: get_motors_id(knee_motor).parse::<i64>().unwrap()};
        let hip_motor = Motor{value: motion, id: get_motors_id(hip_motor).parse().unwrap()};
    }

    //arm location: left or right
    pub fn lift_arm(motion:i64, arm_location: &str){
        let shoulder_motor = Motor{value: motion, id: get_motors_id(arm_location).parse::<i64>().unwrap()};
        let elbow_motor = Motor{value: motion * -1, id: get_motors_id(arm_location).parse::<i64>().unwrap()};
    }

    //arm location: left or right
    pub fn lower_arm(motion: i64, arm_location: &str){
        let shoulder_motor = Motor{value: motion * -1, id: get_motors_id(arm_location).parse::<i64>().unwrap()};
        let elbow_motor = Motor{value: motion, id: get_motors_id(arm_location).parse::<i64>().unwrap()};
    }

    //knuckle location: left or right
    pub fn grip_object(motion: i64, knuckle_location: &str){
        let knuckle_joint = Motor{value: motion * -1, id: get_motors_id(knuckle_location).parse::<i64>().unwrap()};
    }
    
    //knuckle location: left or right
    pub fn release_object(motion: i64, knuckle_location: &str){
        let knuckle_joint = Motor{value: motion, id: get_motors_id(knuckle_location).parse::<i64>().unwrap()};
    }
}


///various actions that the robot can take
///wave left arm or right arm
struct Wave{
    left_arm: bool,
    right_arm: bool
}

struct Step{
    raise: bool,
    lower: bool,
    neutral: bool
}
enum Activity{
    walk,
    run,
    crouch,
    Jump(Jump)
}
enum Jump{
    jump_up,
    jump_forward,
    jump_back
}

enum Action{
    lift,
    lower,
    
}
//walk
//speed: the speed of walking 1f is SLOW 2f is FAST
//direction: the direction in terms of degrees(360)
fn walk(speed: f64, direction: f64){
    //lift leg
    task::spawn(async move{
        task::sleep(Duration::from_secs(2)).await;
        //code to run
        Motor::lift_leg(2, "left");
    });
    //move leg forward
    task::spawn(async move{
        task::sleep(Duration::from_secs(2)).await;
        Motor::move_leg_forward(2, "left");
    });
//    if speed <= 12.0 {
        raise_leg(speed, Activity::walk);//Step{raise: true, lower:false, neutral: false});
        move_step_forward(speed);
        lower_leg(speed);
 //   }else if speed >12.0{
 //       walk_step = HIGH;
//    }
}
//speed is the speed of the motor controlling the joint
//activity determines knee movement
//  -walk
//  -run
//  -kick
/*replaced by Motor::lift_leg
fn raise_leg(speed: f64, activity: Activity){
//    let mut walk_step:Step = Step{raise: true, lower: false, neutral: false};
    //  choose the rate of electrons/speed
    match activity{
            Activity::walk => Motor::move_motor(5, 3),
    //            Motor{hip: 5, knee: 2.5};
    //        }
            Activity::run => Motor::move_motor(10, 5),//{hip: 10, knee: 5};
            crate::Activity::Jump(jump_up) => Motor::move_motor(2, 10),
            Activity::crouch => Motor::move_motor(7, 10),
        }
    } */
/*     let hip : Motor = Motor{
        hip: 5, //turn on motor
        knee: 0
    }; */
//}
///jump 
/// activity determines knee and hip movement 
/// up , forward, back
fn jump(activity: Activity){
    match activity{
        Activity::jump_up => Motor::move_motor(0, 10),
        Activity::jump_forward => Motor::move_motor(5, 7),
        Activity::jump_back => Motor::move_motor(-3, 4)
    }
    //crouch
    Motor::move_motor(-2, -10);
    //jump
    Motor::move_motor(2, 10);

}
//speed is the speed of the motor controlling the joint

fn move_step_forward(speed: f64){

}

//speed is the speed of the motor controlling the joint
fn lower_leg(speed: f64){

}

//}


////latest
///==main
/// =move
/// -run
/// -walk
/// -turn
/// -jump
/// -duck


///==sub
///=lift leg
///=lower leg
fn leg(activity: Activity){

}
}