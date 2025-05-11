import torch
from PIL import Image
from transformers import AutoModel, AutoTokenizer

class Blip:
    model = ''
    tokenizer = ''
    prompt = ''

    def __init__(self):
        self.model = AutoModel.from_pretrained('openbmb/MiniCPM-V-2_6', trust_remote_code=True,
            attn_implementation='sdpa', torch_dtype=torch.bfloat16) # sdpa or flash_attention_2, no eager
        self.model = self.model.eval().cuda()
        self.tokenizer = AutoTokenizer.from_pretrained('openbmb/MiniCPM-V-2_6', trust_remote_code=True)
        print("Loaded Blip")

    def setprompt(self, prompt):
        self.prompt = prompt

    def inference(self, image):
        image = Image.open('xx.jpg').convert('RGB')
        msgs = [{'role': 'user', 'content': [image, self.prompt]}]
        res = self.model.chat(
            image=None,
            msgs=msgs,
            tokenizer=self.tokenizer
        )
        return res

# ## if you want to use streaming, please make sure sampling=True and stream=True
# ## the model.chat will return a generator
# res = model.chat(
#     image=None,
#     msgs=msgs,
#     tokenizer=tokenizer,
#     sampling=True,
#     stream=True
# )

# generated_text = ""
# for new_text in res:
#     generated_text += new_text
#     print(new_text, flush=True, end='')
