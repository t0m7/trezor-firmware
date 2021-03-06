# Automatically generated by pb2py
# fmt: off
from .. import protobuf as p

if __debug__:
    try:
        from typing import Dict, List  # noqa: F401
        from typing_extensions import Literal  # noqa: F401
    except ImportError:
        pass


class WebAuthnCredential(p.MessageType):

    def __init__(
        self,
        index: int = None,
        id: bytes = None,
        rp_id: str = None,
        rp_name: str = None,
        user_id: bytes = None,
        user_name: str = None,
        user_display_name: str = None,
        creation_time: int = None,
        hmac_secret: bool = None,
        use_sign_count: bool = None,
        algorithm: int = None,
        curve: int = None,
    ) -> None:
        self.index = index
        self.id = id
        self.rp_id = rp_id
        self.rp_name = rp_name
        self.user_id = user_id
        self.user_name = user_name
        self.user_display_name = user_display_name
        self.creation_time = creation_time
        self.hmac_secret = hmac_secret
        self.use_sign_count = use_sign_count
        self.algorithm = algorithm
        self.curve = curve

    @classmethod
    def get_fields(cls) -> Dict:
        return {
            1: ('index', p.UVarintType, 0),
            2: ('id', p.BytesType, 0),
            3: ('rp_id', p.UnicodeType, 0),
            4: ('rp_name', p.UnicodeType, 0),
            5: ('user_id', p.BytesType, 0),
            6: ('user_name', p.UnicodeType, 0),
            7: ('user_display_name', p.UnicodeType, 0),
            8: ('creation_time', p.UVarintType, 0),
            9: ('hmac_secret', p.BoolType, 0),
            10: ('use_sign_count', p.BoolType, 0),
            11: ('algorithm', p.SVarintType, 0),
            12: ('curve', p.SVarintType, 0),
        }
