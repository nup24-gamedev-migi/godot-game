class_name TestScript
extends Node2D

const ROT_SPEED: Node2D = 2 * PI / 3

func _ready():
	print("test script is ready")


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float):
	rotate(ROT_SPEED * delta)
