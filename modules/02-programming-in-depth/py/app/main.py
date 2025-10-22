from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from .utils import normalize_user_input

app = FastAPI()

class NormalizeRequest(BaseModel):
    input: str

class NormalizeResponse(BaseModel):
    normalized: str

@app.post("/normalize", response_model=NormalizeResponse)
async def normalize(body: NormalizeRequest):
    text = body.input.strip()
    if not text:
        raise HTTPException(status_code=400, detail="input must not be empty")
    return NormalizeResponse(normalized=normalize_user_input(text))
