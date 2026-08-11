import sys
import json
import urllib.request

from prompt_loader import load_prompt


SERVER_URL = "http://127.0.0.1:8080/v1/chat/completions"


def ask_sira(mode, user_request):

    system_prompt = load_prompt(mode)

    data = {
        "messages": [
            {
                "role": "system",
                "content": system_prompt
            },
            {
                "role": "user",
                "content": user_request
            }
        ],
        "temperature": 0.7,
        "top_p": 0.9,
        "max_tokens": 1000
    }


    request = urllib.request.Request(
        SERVER_URL,
        data=json.dumps(data).encode("utf-8"),
        headers={
            "Content-Type": "application/json"
        }
    )


    with urllib.request.urlopen(request) as response:

        result = json.loads(
            response.read().decode("utf-8")
        )

        print(
            result["choices"][0]["message"]["content"]
        )



if __name__ == "__main__":

    if len(sys.argv) < 3:
        print(
            "Usage: python3 sira_client.py <mode> <request>"
        )
        exit()


    mode = sys.argv[1]

    request = sys.argv[2]


    ask_sira(
        mode,
        request
    )
