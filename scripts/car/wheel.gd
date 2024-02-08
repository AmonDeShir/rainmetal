extends RayCast3D
class_name Wheel

enum STEERING_TYPE {NONE, LEFT, RIGHT }

@export var use_as_steering: STEERING_TYPE = STEERING_TYPE.NONE

var car: Car
var previus_spring_length: float
var start_mesh_y_angle: float

func _ready():
	car = get_parent().get_parent().car
	add_exception(car)
	previus_spring_length = car.suspension_rest_distance / 3
	
	if not use_as_steering:
		$debug_mesh.hide()
	
	if $mesh != null:
		start_mesh_y_angle = $mesh.rotation.y


func _physics_process(delta: float):
	if is_colliding():
		var collision_point := get_collision_point()
		suspension(delta, collision_point)
	
	update_wheel_visuals(delta)


func suspension(delta: float, raycast_dest: Vector3):
		var susp_direction := global_basis.y
		
		var raycast_origin := global_position
		var distance := raycast_dest.distance_to(raycast_origin)

		var spring_length := clampf(distance - car.wheel_radius, 0.2, car.suspension_rest_distance)
		var spring_force := car.spring_strength * (car.suspension_rest_distance - spring_length)
		var spring_velocity := (previus_spring_length - spring_length) / delta
		
		var damper_force := car.spring_damper * spring_velocity
		var suspension_force := basis.y * (spring_force + damper_force)
		
		previus_spring_length = spring_length 
		
		var point := get_wheel_point(raycast_dest)
		
		car.apply_force(susp_direction * suspension_force, point - car.global_position)
		
		if car.debug:
			DebugDraw3D.draw_line(point, point + Vector3(0, spring_length, 0), Color.BLACK, 0)
			DebugDraw3D.draw_arrow(global_position, to_global(position + Vector3(-position.x, (spring_velocity), -position.z)), Color.PINK, 0.1, true)
			DebugDraw3D.draw_arrow(global_position, to_global(position + Vector3(-position.x, (suspension_force.y / (2 * car.mass)), -position.z)), Color.YELLOW, 0.1, true)


func get_wheel_point(collision_point: Vector3) -> Vector3:
	return collision_point + Vector3(0, car.wheel_radius, 0)


func update_wheel_visuals(delta: float):
	if $mesh == null:
		return
	
	if is_colliding():
		$mesh.position.y = to_local(get_collision_point()).y + car.wheel_radius
	else:
		$mesh.position.y = 0
	
	if use_as_steering != STEERING_TYPE.NONE:
		$debug_mesh.visible = car.debug
		
		if Input.get_axis("turn_right", "turn_left") != 0:
			$mesh.rotation.y = lerp_angle($mesh.rotation.y, car.get_steering_angle() + start_mesh_y_angle, car.steering_speed * delta)
		else:
			$mesh.rotation.y = lerp_angle($mesh.rotation.y, car.get_steering_angle() + start_mesh_y_angle, car.steering_speed * 2 * delta)
	
	var speed = car.get_car_speed_ms() / (2 * PI * car.wheel_radius)
	var wheel_rotation_angle = speed * 2 * PI * delta
	$mesh.rotate_x(-wheel_rotation_angle)
	$mesh.rotation.z = 0


func get_distance_to_ground() -> float:
	return get_collision_point().distance_to(global_position)
