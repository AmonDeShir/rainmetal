extends RigidBody3D
class_name Car

const MS_TO_KMS := 3.6
const HP_TO_NMS := 745
const UNSTUCK_TIME := 0.1
const ROTATION_REMOVING_TIME := 0.05

@export var car_name: String

@export var suspension_rest_distance: float = 0.5
@export var spring_strength: float = 10
@export var spring_damper: float = 1.0
@export var wheel_radius: float = 0.33

@export var engine_power: float
@export var max_speed: float

@export var max_steering_angle: Curve
@export var steering_speed: float = 10

@export var speedometer: Speedometer

@export var debug: bool = false
@onready var debugPanel: CarDebugInfo = $debug_ui

var time_to_unstuck := UNSTUCK_TIME
var collider: PhysicsBody3D
var collision_normal

var wheels: Array[Wheel] = []
var time_to_remove_rotation := ROTATION_REMOVING_TIME


func _ready():
	engine_power = engine_power * HP_TO_NMS
	set_debug(debug)
	find_wheels()


func set_debug(value: bool, node: Node = self):
	if node == self:
		debug = value
	
	for child in node.get_children():
		if child.name == "mesh":
			child.visible = !value
		elif child.name == "debug_mesh" or child.name == "debug_ui":
			child.visible = value
		else:
			set_debug(value, child)


func find_wheels():
	for part in get_children():
		if part is CarPart:
			wheels.append_array(part.get_wheels())


func _physics_process(delta):
	if not is_on_ground():
		return
	
	unflip(delta)
	unstuck(delta)
	accelerate()
	steering()
	remove_after_steering_rotation(delta)
	steering_friction(delta)
	friction()

	if speedometer != null:
		speedometer.set_speed(get_car_speed_kms())


func unflip(delta: float):
	if rotation.z < deg_to_rad(30):
		return
	
	var rot_speed = lerpf(0.001, 0.01, rad_to_deg(rotation.z)/90) * mass * delta
	rotation.z = lerp_angle(rotation.z, 0,  rot_speed)


func unstuck(delta: float):
	const STUCK_DISTANCE = 0.1

	if collider == null or collision_normal == null:
		return
		
	if collision_normal != global_basis.y:
		return
	
	var stuck = true
	var first_wheel = wheels.front().get_distance_to_ground()
	
	for i in range(1, len(wheels)):
		if abs(first_wheel - wheels[i].get_distance_to_ground()) > STUCK_DISTANCE:
			stuck = false
	
	if stuck:
		time_to_unstuck -= delta
	else:
		time_to_unstuck = UNSTUCK_TIME
	
	if time_to_unstuck <= 0:
		time_to_unstuck = 0.1
		translate(Vector3(0, 0.5, 0))


func is_on_ground() -> bool:
	for wheel in wheels:
		if wheel.is_colliding():
			return true
	
	return false


func accelerate():
	var axel_input = Input.get_axis("backward", "forward")
	var driving_direction := Quaternion(transform.basis) * Vector3.FORWARD
	var acceleration_force: float = axel_input * 10000

	apply_central_force(driving_direction * acceleration_force)
	
	if debug:
		DebugDraw3D.draw_arrow(global_position, to_global(Vector3(0, 0, -axel_input * 10000 / mass)), Color.BLUE_VIOLET, 0.1, true)
		debugPanel.set_acceleration(acceleration_force)


func steering():
	var steering_direction := Quaternion(transform.basis) * Vector3.UP
	var steering_force := 50000.0
	
	if Input.is_action_pressed("turn_left"):
		apply_torque(steering_direction * steering_force)

	if Input.is_action_pressed("turn_right"):
		apply_torque(-steering_direction * steering_force)


func remove_after_steering_rotation(delta: float):
	if Input.get_axis("turn_right", "turn_left") == 0:
		time_to_remove_rotation = min(time_to_remove_rotation + delta, ROTATION_REMOVING_TIME)
		angular_velocity.y = lerpf(angular_velocity.y, 0, time_to_remove_rotation)


func steering_friction(delta: float):
	var direction := global_basis.x
	var steering_vel := direction.dot(linear_velocity) 
	var acceleration = steering_vel / delta
	var force = acceleration * mass / 2
	
	if abs(steering_vel) < 0.05:
		return
	
	apply_central_force(-direction * force)
	
	if debug:
		DebugDraw3D.draw_arrow(global_position, global_position + (direction * force) / mass, Color.BLACK, 0.1, true)
		debugPanel.set_steering(force)


func friction():
	if Input.is_action_pressed("forward"):
		return
	
	var direction := global_basis.z

	# add here more advance friction!
	var force = direction.dot(linear_velocity) * (mass)
	
	apply_central_force(-direction * force)
	
	if debug:
		DebugDraw3D.draw_arrow(global_position, global_position + (-direction * force) / mass, Color.SANDY_BROWN, 0.1, true)
		debugPanel.set_friction(force)


func _input(event):
	if (event.is_action("debug") and event.is_released()):
		debug = !debug
		set_debug(debug, self)


func get_car_speed_ms():
	return linear_velocity.length()


func get_car_speed_kms():
	return linear_velocity.length() * MS_TO_KMS


func _integrate_forces(state : PhysicsDirectBodyState3D) -> void:
	if state.get_contact_count() > 0:
		collision_normal = state.get_contact_local_normal(0)
	else:
		collision_normal = null


func _on_body_entered(body):
	print("colliding with: ", body)
	collider = body


func _on_body_exited(body):
	print("not colliding with: ", body)
	if collider == body:
		collider = null


func get_steering_angle() -> float:
	return deg_to_rad(max_steering_angle.sample(get_car_speed_kms() / max_speed)) * Input.get_axis("turn_right", "turn_left")
