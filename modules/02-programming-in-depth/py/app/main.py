from fastapi import FastAPI, HTTPException, Request
from pydantic import BaseModel
from .utils import normalize_user_input
import uuid
import time

app = FastAPI()

class NormalizeRequest(BaseModel):
    input: str

class NormalizeResponse(BaseModel):
    normalized: str

@app.middleware("http")
async def add_request_id(request: Request, call_next):
    req_id = str(uuid.uuid4())
    start = time.time()
    response = await call_next(request)
    duration_ms = int((time.time() - start) * 1000)
    response.headers["X-Request-Id"] = req_id
    response.headers["X-Response-Time"] = str(duration_ms)
    return response

@app.post("/normalize", response_model=NormalizeResponse)
async def normalize(body: NormalizeRequest):
    text = body.input.strip()
    if not text:
        raise HTTPException(status_code=400, detail="input must not be empty")
    return NormalizeResponse(normalized=normalize_user_input(text))
