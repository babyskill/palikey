#include "context_state.h"

ContextManager::ContextManager() {
}

ContextManager::~ContextManager() {
    Clear();
}

ContextState* ContextManager::GetOrCreate(ITfContext *pic, const PalikeyConfig &config) {
    if (!pic) return nullptr;

    auto it = m_contexts.find(pic);
    if (it != m_contexts.end()) {
        return &it->second;
    }

    ContextState state;
    state.pContext = pic;
    state.engine = palikey_new(config.scheme, config.niggahita, config.extended_set);
    state.pComposition = nullptr;
    state.inComposition = false;

    m_contexts[pic] = state;
    return &m_contexts[pic];
}

ContextState* ContextManager::Find(ITfContext *pic) {
    if (!pic) return nullptr;
    auto it = m_contexts.find(pic);
    if (it != m_contexts.end()) {
        return &it->second;
    }
    return nullptr;
}

void ContextManager::Remove(ITfContext *pic) {
    if (!pic) return;

    auto it = m_contexts.find(pic);
    if (it != m_contexts.end()) {
        if (it->second.pComposition) {
            it->second.pComposition->Release();
            it->second.pComposition = nullptr;
        }
        if (it->second.engine) {
            palikey_free(it->second.engine);
            it->second.engine = nullptr;
        }
        m_contexts.erase(it);
    }
}

void ContextManager::Clear() {
    for (auto &pair : m_contexts) {
        if (pair.second.pComposition) {
            pair.second.pComposition->Release();
            pair.second.pComposition = nullptr;
        }
        if (pair.second.engine) {
            palikey_free(pair.second.engine);
            pair.second.engine = nullptr;
        }
    }
    m_contexts.clear();
}

void ContextManager::UpdateConfig(const PalikeyConfig &config) {
    for (auto &pair : m_contexts) {
        if (pair.second.engine) {
            palikey_set_scheme(pair.second.engine, config.scheme);
        }
    }
}

void ContextManager::OnCompositionTerminated(ITfComposition *pComposition) {
    if (!pComposition) return;
    for (auto &pair : m_contexts) {
        if (pair.second.pComposition == pComposition) {
            pair.second.pComposition->Release();
            pair.second.pComposition = nullptr;
            pair.second.inComposition = false;
            if (pair.second.engine) {
                palikey_reset_instance(pair.second.engine);
            }
            break;
        }
    }
}
