extends Node
class_name CarDebugInfo

@onready
var car: Car = get_parent()

var acceleration_force := 0.0
var steering_force := 0.0
var friction_force := 0.0


func _process(_delta: float) -> void:
	$info.text = ""
	$info.text += "Car: " + car.car_name + " Parts: " + parts_to_str() + "\n"
	$info.text += "Mass: %.0f\n" % car.mass
	$info.text += "Engine Power: %.2f N\\S (%.0f HP)\n" % [car.engine_power, car.engine_power / car.HP_TO_NMS]
	$info.text += "Angural Velocity: %.3f\n" % car.angular_velocity.y
	$info.text += "Speed: %.0f km\\h (%.2f m\\s)\n" % [car.get_car_speed_kms(), car.get_car_speed_ms()]
	$info.text += "Acceleration force: %.3f N\n" % acceleration_force
	$info.text += "Steering force: %.3f N\n" % steering_force
	$info.text += "Friction force: %.3f N\n" % friction_force


func parts_to_str() -> String:
	var parts := []
	
	for part in car.get_children():
		if part is CarPart:
			parts.append(part.part_name)
	
	return ", ".join(parts)


func set_acceleration(value: float):
	acceleration_force = value


func set_steering(value: float):
	steering_force = value


func set_friction(value: float):
	friction_force = value
	
