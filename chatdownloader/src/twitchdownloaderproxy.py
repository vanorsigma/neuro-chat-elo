"""
A command proxy to: https://github.com/lay295/TwitchDownloader.

I'm using this to pay homeage to one of the contributors of
TwitchDownloader, who is a fan of Neuro

Ensures that the latest executable for TwitchDownloader is downloaded
and used.

Note: Will always download the AMD64 executable. This script is not
meant to run on any other environments anyway
"""

import tempfile
import requests
import os
import zipfile
import json
from typing import Optional

RELEASES_URL = "https://api.github.com/repos/lay295/TwitchDownloader/releases/latest"


class TwitchChatDownloader:
    """
    Initializes the TwitchChatDownloader object. If executable_path is
    provided, it will be used instead of downloading the executable.

    :param executable_path: The path to the executable
    """

    def __init__(self, executable_path=""):
        # We don't delete on close because we need to use the executable
        # Instead, it'll have to be deleted manually
        # (our target is Python 3.11, which does not have delete_on_close yet)
        self._predownloaded = executable_path != ""
        self._downloaded = self._predownloaded

        self.executable_tempfile_name = (
            tempfile.mktemp() if not self._downloaded else executable_path
        )
        self.executable_tempfile = open(self.executable_tempfile_name, "ab+")
        self.executable_tempfile.close()

        self.chat_tempfile_name = tempfile.mktemp(".json")
        self.chat_tempfile = open(self.chat_tempfile_name, "ab+")
        self.chat_tempfile.close()
        os.remove(self.chat_tempfile_name)

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.__delete_tmps()

    def __delete_tmps(self):
        self.executable_tempfile.close()
        self.chat_tempfile.close()
        if not self._predownloaded:
            os.remove(self.executable_tempfile_name)
        os.remove(self.chat_tempfile_name)

    @staticmethod
    def __parse_github_api_response(response) -> Optional[str]:
        download_url = None
        for asset in response["assets"]:
            if asset["name"].endswith("Linux-x64.zip"):
                download_url = asset["browser_download_url"]
                break
        return download_url

    def download_executable(self):
        """
        Downloads the latest executable from the releases page
        """
        r = requests.get(RELEASES_URL)
        if r.status_code != 200:
            raise RuntimeError("Could not fetch releases page")

        body = r.json()
        if "assets" not in body:
            raise RuntimeError("No assets found in the release")

        download_url = self.__parse_github_api_response(body)
        if download_url is None:
            raise RuntimeError("Could not find the download link")

        r = requests.get(download_url)
        if r.status_code != 200:
            raise RuntimeError("Could not download the executable")

        with tempfile.TemporaryFile() as tempf:
            tempf.write(r.content)
            with zipfile.ZipFile(tempf) as zipf:
                zipf.extract(
                    "TwitchDownloaderCLI",
                    path=os.path.dirname(self.executable_tempfile_name),
                )
                os.rename(
                    os.path.join(
                        os.path.dirname(self.executable_tempfile_name),
                        "TwitchDownloaderCLI",
                    ),
                    self.executable_tempfile_name,
                )

        try:
            os.chmod(self.executable_tempfile_name, 0o700 | os.X_OK)
        except Exception as e:
            raise RuntimeError("Could not make the executable executable")

        self._downloaded = True

    # TODO: Don't use object, use an actual type. Probably do a dataclass
    def download_chat(self, vod_id: str) -> object:
        """
        Downloads the chat for a given VOD ID. Requires the executable
        to be downloaded.
        """
        if not self._downloaded:
            print("No executable downloaded, downloading...")
            self.download_executable()

        exit_code = os.system(
            f"{self.executable_tempfile_name} chatdownload -u {vod_id} -o {self.chat_tempfile_name}"
        )
        if exit_code != 0:
            raise RuntimeError("Error with chat download")

        with open(self.chat_tempfile_name, "r", encoding='utf8') as f:
            json_data = json.load(f)

        return json_data


if __name__ == "__main__":
    with TwitchChatDownloader() as tcd:
        d = tcd.download_chat("2170316549")
    with open('output.json', 'w', encoding='utf8') as f:
        json.dump(d, f)
