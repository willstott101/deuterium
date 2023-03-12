import pytest
from deuterium import Vector3


def test_construction():
    assert Vector3(1.0, 2.0, 3.0)
    assert Vector3(1, 2, 3) == Vector3(1.0, 2.0, 3.0)
    assert Vector3(1) == Vector3(1, 0, 0)
    assert Vector3(1, 2) == Vector3(1, 2, 0)
    assert Vector3(x=1) == Vector3(1, 0, 0)
    assert Vector3(y=2) == Vector3(0, 2, 0)
    assert Vector3(z=3) == Vector3(0, 0, 3)
    assert Vector3() == Vector3(0, 0, 0)
    # assert Vector3.from_seq([1, 2, 3, 4]) == Vector3(1, 2, 3)
    # assert Vector3.from_seq([1, 2, 3, 4, 5], 2) == Vector3(3, 4, 5)


def test_repr():
    v = Vector3(1.0, 2.0, 3.0)
    assert repr(v) == "Vector3(1, 2, 3)"
    assert repr(v) == str(v)


def test_equality():
    assert Vector3(1, 0, 0) != Vector3(0, 0, 0)
    assert Vector3(0, 1, 0) != Vector3(0, 0, 0)
    assert Vector3(0, 0, 1) != Vector3(0, 0, 0)

    assert Vector3(0, 0, 0.000001) != Vector3(0, 0, 0)
    assert not Vector3(0, 0, 0.000001).approx_equals(Vector3(0, 0, 0))
    assert Vector3(0, 0, 0.0000000001) != Vector3(0, 0, 0)
    assert Vector3(0, 0, 0.0000000001).approx_equals(Vector3(0, 0, 0))


def test_indexing():
    assert Vector3(1, 2, 3)[0] == 1
    assert Vector3(1, 2, 3)[1] == 2
    assert Vector3(1, 2, 3)[2] == 3
    assert Vector3(1, 2, 3)[-1] == 3
    assert Vector3(1, 2, 3)[-2] == 2
    assert Vector3(1, 2, 3)[-3] == 1

    with pytest.raises(IndexError):
        Vector3(1, 2, 3)[3]

    with pytest.raises(IndexError):
        Vector3(1, 2, 3)[-4]

    assert Vector3(1, 2, 3).x == 1
    assert Vector3(1, 2, 3).y == 2
    assert Vector3(1, 2, 3).z == 3


def test_index_mutation():
    v = Vector3(1, 2, 3)
    v[0] = 99
    assert v == Vector3(99, 2, 3)
    v[1] = -2
    assert v == Vector3(99, -2, 3)
    v[2] = 12
    assert v == Vector3(99, -2, 12)
    v.x = -1
    assert v == Vector3(-1, -2, 12)
    v.y = -1
    assert v == Vector3(-1, -1, 12)
    v.z = -1
    assert v == Vector3(-1, -1, -1)



def test_iter():
    assert len(Vector3(0, 0, 0)) == 3
    assert list(Vector3(1, 2, 3)) == [1, 2, 3]


def test_length_and_normalization():
    assert Vector3(2, 0, 0).length() == 2
    assert Vector3(2, 0, 0).length_squared() == 4
    assert Vector3(2, 0, 0).normalized() == Vector3(1, 0, 0)
    v = Vector3(0, 0.5, 0)
    v.normalize()
    assert v == Vector3(0, 1, 0)


def test_simple_ops():
    assert Vector3(1, 2, 3) + Vector3(10, 20, 30) == Vector3(11, 22, 33)
    assert Vector3(1, 2, 3) - Vector3(10, 20, 30) == Vector3(-9, -18, -27)
    v = Vector3(1, 2, 3)
    v += Vector3(10, 20, 30)
    assert v == Vector3(11, 22, 33)
    v -= Vector3(10, 20, 30)
    assert v == Vector3(1, 2, 3)
    assert v * 5 == Vector3(5, 10, 15)
    v *= 10
    assert v == Vector3(10, 20, 30)
    assert -v == Vector3(-10, -20, -30)
    v.negate()
    assert v == Vector3(-10, -20, -30)
    v /= 10
    assert v == Vector3(-1, -2, -3)
    assert v / -2 == Vector3(0.5, 1, 1.5)


def test_distances():
    assert Vector3(0, 1, 0).distance_to(Vector3(0, -20, 0)) == 21
    assert Vector3(0, 1, 0).distance_to_squared(Vector3(0, -1, 0)) == 4


def test_cross():
    assert Vector3(3).cross(Vector3(0, 6, 0)) == Vector3(0, 0, 18)


def test_dot():
    assert Vector3(3).dot(Vector3(0, 6, 0)) == 0
    assert Vector3(1, 2, 3).dot(Vector3(0.1, 0.2, 0.3)) == 1.4

