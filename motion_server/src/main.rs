use franka::{FrankaResult, JointPositions, MotionFinished, Robot, RobotState};
use std::f64::consts::PI;
use std::time::Duration;
fn main() -> FrankaResult<()> {
  let mut robot = Robot::new("robotik-bs.de", None, None)?;
  robot.set_default_behavior()?;
  println!("WARNING: This example will move the robot! Please make sure to have the user stop button at hand!");
  println!("Press Enter to continue...");
  std::io::stdin().read_line(&mut String::new()).unwrap();

  // Set additional parameters always before the control loop, NEVER in the control loop!
  // Set collision behavior.
  robot.set_collision_behavior(
      [20.0, 20.0, 18.0, 18.0, 16.0, 14.0, 12.0],
      [20.0, 20.0, 18.0, 18.0, 16.0, 14.0, 12.0],
      [20.0, 20.0, 18.0, 18.0, 16.0, 14.0, 12.0],
      [20.0, 20.0, 18.0, 18.0, 16.0, 14.0, 12.0],
      [20.0, 20.0, 20.0, 25.0, 25.0, 25.0],
      [20.0, 20.0, 20.0, 25.0, 25.0, 25.0],
      [20.0, 20.0, 20.0, 25.0, 25.0, 25.0],
      [20.0, 20.0, 20.0, 25.0, 25.0, 25.0],
  )?;

  let q_goal = [0., -PI / 4., 0., -3. * PI / 4., 0., PI / 2., PI / 4.];
  robot.joint_motion(0.5, &q_goal)?;
  println!("Finished moving to initial joint configuration.");
  let mut initial_position = JointPositions::new([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
  let mut time = 0.;
  let callback = |state: &RobotState, time_step: &Duration| -> JointPositions {
      time += time_step.as_secs_f64();
      if time == 0. {
          initial_position.q = state.q_d;
      }
      let mut out = JointPositions::new(initial_position.q);
      let delta_angle = PI / 8. * (1. - f64::cos(PI / 2.5 * time));
      out.q[3] += delta_angle;
      out.q[4] += delta_angle;
      out.q[6] += delta_angle;
      if time >= 5.0 {
          println!("Finished motion, shutting down example");
          return out.motion_finished();
      }
      out
  };
  robot.control_joint_positions(callback, None, None, None)
}
