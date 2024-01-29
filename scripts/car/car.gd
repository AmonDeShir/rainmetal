extends VehicleBody3D
class_name Car

const MAX_STEER = 0.8;
const STEER_SPEED = 10

@export
var current_gear : int = 1

@export
var max_engine_force : float = 1000.0

@export
var gear_ratios : Array = [0.5, 0.7, 1.0, 1.5, 2.0]


func shift_up():
	if current_gear < len(gear_ratios):
		current_gear += 1


func shift_down():
	if current_gear > len(gear_ratios):
		current_gear -= 1


func _process(delta):
	steering = move_toward(steering, Input.get_axis("turn_right", "turn_left") * MAX_STEER, delta * STEER_SPEED)
	# engine_force = max_engine_force * gear_ratios[current_gear - 1]


func _input(event):
	if event.is_action_pressed("shift_up"):
		shift_up()
	elif event.is_action_pressed("shift_down"):
		shift_down()
