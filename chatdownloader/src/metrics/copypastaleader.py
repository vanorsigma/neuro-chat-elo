"""
The copypasta leader metric.

TODO: This guy is the reason why I can't do multi-processing just
yet. Need a merge() function at some point to consolidate data between
multiple processes somehow.
"""

import heapq
import itertools
from _types import Comment

from .abstractmetric import AbstractMetric

WEIGHT_COPYPASTA = 0.3
CHAIN_GRACE = 5
MATCHING_THRESHOLD = 0.8

class CopypastaLeader:
    """
    The Copypasta leader metric.

    TODO: You probably need to rewrite this
    """
    def __init__(self):
        self.__heap = []

    @staticmethod
    def _lcs(lhs: str, rhs: str) -> int:
        # the dp approach, copied from somewhere
        m = len(lhs)
        n = len(rhs)

        L = [[None] * (n + 1) for i in range(m + 1)]

        for i in range(m + 1):
            for j in range(n + 1):
                if i == 0 or j == 0:
                    L[i][j] = 0
                elif lhs[i - 1] == rhs[j - 1]:
                    L[i][j] = L[i - 1][j - 1] + 1
                else:
                    L[i][j] = max(L[i - 1][j] , L[i][j - 1])

        return L[m][n]

    @staticmethod
    def padTo(target: str, padding: str, maxlen: int) -> str:
        return (target + (padding *
                          ((maxlen - len(target)) // len(padding) + 1))
                )[:maxlen]

    @classmethod
    def get_name(cls):
        return 'copypasta'

    def get_metric(self, comment: Comment,
                   sequence_no: int) -> list[dict[str, int]]:
        text = ' '.join(
            fragment.text for fragment in comment.message.fragments)

        # if empty heap then just go for it
        if len(self.__heap) == 0:
            heapq.heappush(self.__heap, (sequence_no, text,
                                         comment.commenter._id,
                                         sequence_no))

        # go through everything in the heap and find the best matching string
        matching_scores = [self._lcs(
            self.padTo(item[1], item[1],
                       len(text)), text) // len(text)
                           for item in self.__heap]
        best_match = max(enumerate(self.__heap),
                         key=lambda item: matching_scores[item[0]])

        if matching_scores[best_match[0]] < MATCHING_THRESHOLD:
            # tuple: (last seq number, text, id, original seq number )
            heapq.heappush(self.__heap, (sequence_no, text,
                                         comment.commenter._id,
                                         sequence_no))
        else:
            # in-place updating of the tuple. TODO: find a better way
            # to do this, i'm too lazy atm
            item = self.__heap[best_match[0]]
            self.__heap[best_match[0]] = (
                sequence_no,
                *item[1:]
            )
            heapq.heapify(self.__heap)

        # evict old heap top
        if len(self.__heap) > 0 and self.__heap[0][0] > CHAIN_GRACE:
            item = heapq.heappop(self.__heap)
            return {item[2]: (item[0] - item[3]) * WEIGHT_COPYPASTA}

        return {}
