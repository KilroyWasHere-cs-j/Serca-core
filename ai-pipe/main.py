# main.py
import utils
import pipe
import torch
from pipe import Lava
from colorama import Fore, Back, Style


def main():

    image_path = "images/bagel.jpg"
    output = "NULL"
    prompt, model_id = utils.load()

    lava = pipe.Lava(model_id=model_id, prompt=prompt, path=image_path)
    lava.load_model()
    lava.load_image()
    output = lava.inference()
    print(Fore.GREEN)
    print(output)
    print(Fore.WHITE)
    
if __name__ == "__main__":
    main()
