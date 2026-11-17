#pragma once

#ifndef PALIKEY_TSF_CONTEXT_STATE_H
#define PALIKEY_TSF_CONTEXT_STATE_H

#include <windows.h>
#include <msctf.h>
#include <map>
#include "palikey.h"
#include "palikey_config.h"

struct ContextState {
    ITfContext *pContext = nullptr;
    PalikeyEngine *engine = nullptr;
    ITfComposition *pComposition = nullptr;
    bool inComposition = false;
};

class ContextManager {
public:
    ContextManager();
    ~ContextManager();

    ContextState* GetOrCreate(ITfContext *pic, const PalikeyConfig &config);
    ContextState* Find(ITfContext *pic);
    void Remove(ITfContext *pic);
    void Clear();
    void UpdateConfig(const PalikeyConfig &config);
    void OnCompositionTerminated(ITfComposition *pComposition);

private:
    std::map<ITfContext*, ContextState> m_contexts;
};

#endif // PALIKEY_TSF_CONTEXT_STATE_H
