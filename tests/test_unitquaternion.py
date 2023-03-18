import pytest
from deuterium import UnitQuaternion


def test_constructor():
    assert UnitQuaternion()
    assert UnitQuaternion() == UnitQuaternion.identity()

