# main.py
import utils
import torch
from colorama import Fore, Back, Style
import streamer
import pipe
from pipe import Lava



def main():
    prompt, model_id = utils.load()
    lava = pipe.Lava(model_id=model_id, prompt=prompt)
    lava.load_model()
    print(streamer.run_vid(url="https://a.nwps.fi/15301218273763.mp4", model=lava))
    
if __name__ == "__main__":
    main()
