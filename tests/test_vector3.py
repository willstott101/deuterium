import pytest
from deuterium import Vector3


def test_constructor():
    assert Vector3(1.0, 2.0, 3.0)
    assert Vector3(1, 2, 3) == Vector3(1.0, 2.0, 3.0)
    assert Vector3(1) == Vector3(1, 0, 0)
    assert Vector3(1, 2) == Vector3(1, 2, 0)
    assert Vector3(x=1) == Vector3(1, 0, 0)
    assert Vector3(y=2) == Vector3(0, 2, 0)
    assert Vector3(z=3) == Vector3(0, 0, 3)


def test_repr():
    v = Vector3(1.0, 2.0, 3.0)
    assert repr(v) == "Vector3(1, 2, 3)"
    assert repr(v) == str(v)


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


def test_ops():
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
