extends Node3D
class_name CarPart

var car: Car

@export
var mass: float = 0

@export
var part_name := "Part name"


func _enter_tree():
	car = get_parent()


func _ready():
	car.mass += mass


func get_wheels() -> Array[Wheel]:
	var wheels: Array[Wheel] = []
	
	for child in $wheels.get_children():
		if child is Wheel:
			wheels.append(child)
	
	return wheels
