extends VehicleBody3D
class_name Car

const MAX_STEER = 0.8
const STEER_SPEED = 10

const NEUTRAL_GEAR = 0
const REVERSE_GEAR = -1

@export
var max_breake_force: float = 100

@export
var max_engine_force: float = 50

@export
var gear_ratios: Array = [2.69, 2.01, 1.59, 1.32, 1.13, 1.0]

@export
var reverse_ratio: float = -2.5

@export
var final_drive: float = 3.35

@export
var current_gear: int = 1 # -1 = reverse, 0 = neutral

@export
var speedometer: Speedometer

@onready
var wheel = $wheel_r1

@onready 
var last_pos = position

# speed meter/second
var current_speed_mps: float = 0


func get_speed_kph():
	return current_speed_mps * 3600.0 / 1000.0


func calculate_rpm() -> float:
	if current_gear == NEUTRAL_GEAR:
		return 0
	
	const ROTATION_PER_MINUTE = 60
	
	var wheel_circ = 2 * PI * wheel.wheel_radius
	var wheel_rot_speed = ROTATION_PER_MINUTE * current_speed_mps / wheel_circ 
	var drive_shaft_rot_speed = wheel_rot_speed * final_drive
	
	if current_gear == REVERSE_GEAR:
		return drive_shaft_rot_speed * -reverse_ratio
	
	return drive_shaft_rot_speed * gear_ratios[current_gear - 1]


func shift_up():
	if current_gear < len(gear_ratios):
		current_gear += 1


func shift_down():
	if current_gear > len(gear_ratios):
		current_gear -= 1

func _process(_delta):
	var speed = get_speed_kph()
	var rpm = calculate_rpm()
	
	speedometer.set_speed(speed)
	speedometer.set_rpm(rpm)
	speedometer.set_gear(current_gear)


func _physics_process(delta):
	current_speed_mps = (position - last_pos).length() / delta
	
	steering = move_toward(steering, Input.get_axis("turn_right", "turn_left") * MAX_STEER, delta * STEER_SPEED)
	engine_force = Input.get_axis("backward", "forward") * max_engine_force

	last_pos = position

func _input(event):
	if event.is_action_pressed("shift_up"):
		shift_up()
	elif event.is_action_pressed("shift_down"):
		shift_down()
