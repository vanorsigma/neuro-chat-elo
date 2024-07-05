"""
Processes chat logs using the metrics and exports them via the
leaderboard.
"""

import logging
import os
from datetime import datetime
from typing import Any, Callable

from twitchAPI.twitch import Twitch

from _types import ChatLog, UserChatPerformance
from leaderboards import EXPORTED_LEADERBOARDS
from metadata import EXPORTED_METADATA
from metrics import EXPORTED_METRICS

debug_mode = os.getenv('DEBUG')


class ChatLogProcessor:
    """
    Processes the chat logs.

    The class uses the metrics package to extract metrics from the
    chat messages, the metadata package to extract any user-metadata,
    and the leaderboards package to export the metrics / required user
    metadata to the right people
    """
    def __init__(self, twitch: Twitch) -> None:
        self.twitch = twitch

    @staticmethod
    def _debug_timing(tag: str, func: Callable[[], None], *args) -> Any:
        if debug_mode:
            start_time = datetime.now()
            ret_val = func(*args)
            end_time = datetime.now()
            logging.debug('%s took %f us to calculate',
                          tag, (end_time - start_time).microseconds)
            return ret_val
        return func(*args)

    def _parse_to_log_object(self, chat_log_path: str) -> ChatLog:
        with open(chat_log_path, "r", encoding="utf8") as f:
            chat_logs = ChatLog.from_json(f.read())

        return chat_logs

    def parse_from_dict(self, data: dict[Any]) -> list[UserChatPerformance]:
        """
        Gets each user's chat performance from the dictionary.

        Ideally, the dictionary should have the same format as seen in ChatLog.
        :param: data The dicitonary in quesiton
        :returns: The same return value as parse
        """
        return self.parse_from_log_object(ChatLog.from_dict(data))

    def parse_from_log_object(self, chatlog: ChatLog) -> list[UserChatPerformance]:
        """
        From a ChatLog, gets all possible ChatPerformances

        :param: chatlog The ChatLog
        :returns: The same return value as parse
        """
        start_time = datetime.now()
        # TODO: Has a bunch of code duplication that I would rather
        # not have
        pre_performance = {}

        # initialize all the metric & metadata classes
        metric_instances = [m() for m in EXPORTED_METRICS]
        metadata_instances = [m(self.twitch) for m in EXPORTED_METADATA]

        for seq_no, comment in enumerate(chatlog.comments):
            logging.debug("Processing comment by %s (message %d of %d)",
                          comment.commenter.display_name, seq_no + 1,
                          len(chatlog.comments))
            if comment.commenter._id not in pre_performance:
                pre_performance[comment.commenter._id] = UserChatPerformance(
                    id=comment.commenter._id,
                    username=comment.commenter.display_name,
                    avatar=comment.commenter.logo,
                    metrics={m.get_name(): 0 for m in metric_instances},
                    metadata={m.get_name():
                              m.get_default_value() for m in metadata_instances}
                )

            metric_update_arr = {
                metric.get_name(): self._debug_timing(
                    metric.get_name(),
                    metric.get_metric,
                    comment,
                    seq_no)
                for metric in metric_instances
            }

            metadata_update_arr = {
                metadata.get_name(): self._debug_timing(
                    metadata.get_name(),
                    metadata.get_metadata,
                    comment,
                    seq_no)
                for metadata in metadata_instances
            }

            logging.debug('Metric Update Array: %s', metric_update_arr)
            logging.debug('Metadata Update Array: %s', metadata_update_arr)

            for k, v in metric_update_arr.items():
                for user_id, met_value in v.items():
                    # NOTE: the user_id will definitely exist
                    pre_performance[user_id].metrics[k] += met_value

            for k, v in metadata_update_arr.items():
                for user_id, meta_value in v.items():
                    pre_performance[user_id].metadata[k] = meta_value

        # finalize
        metric_update_arr = {
            metric.get_name(): self._debug_timing(
                metric.get_name(), metric.finish)
            for metric in metric_instances
        }

        logging.debug('Finalize - Metric Update Array: %s', metric_update_arr)

        for k, v in metric_update_arr.items():
            for user_id, met_value in v.items():
                # NOTE: the user_id will definitely exist
                pre_performance[user_id].metrics[k] += met_value

        logging.debug('Chat log processor took %f seconds to process the logs',
                      (datetime.now() - start_time).total_seconds())

        return pre_performance.values()

    def parse(self, chat_log_path: str) -> list[UserChatPerformance]:
        """
        Given a chat log generated by the Twitch Downloader CLI tool,
        parses the messages and produces a list connecting a user to metrics

        :param chat_log_path: The path to the chat log file
        :return: A list of {UserChatPerformance} objects
        """
        return self.parse_from_log_object(
            self._parse_to_log_object(chat_log_path))

    @classmethod
    def export_to_leaderboards(cls, performances: list[UserChatPerformance]):
        leaderboards = [l() for l in EXPORTED_LEADERBOARDS]
        for leaderboard in leaderboards:
            for performance in performances:
                leaderboard.update_leaderboard(performance)
            leaderboard.save()

if __name__ == '__main__':
    clp = ChatLogProcessor()
    result = clp.parse('src/result.json')
    clp.export_to_leaderboards(result)
