from deuterium import *


def test_mat4_vec3():
    m = Matrix4.identity()
    v = Vector3(1, 2, 3)
    assert m * v == Vector3(1, 2, 3)

