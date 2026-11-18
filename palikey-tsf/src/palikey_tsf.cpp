#include "palikey_tsf.h"
#include "guids.h"
#include "edit_session.h"
#include "display_attribute.h"
#include "easy_unicode.h"
#include <cwchar>

CPaliKeyTextService::CPaliKeyTextService()
    : m_cRef(1),
      m_pThreadMgr(nullptr),
      m_tfClientId(TF_CLIENTID_NULL),
      m_dwActivateFlags(0),
      m_threadMgrEventSinkCookie(TF_INVALID_COOKIE),
      m_displayAttrAtom(TF_INVALID_GUIDATOM) {
    palikey_config_init_default(&m_config);
}

CPaliKeyTextService::~CPaliKeyTextService() {
}

STDMETHODIMP CPaliKeyTextService::QueryInterface(REFIID riid, void **ppvObj) {
    if (!ppvObj) return E_INVALIDARG;
    *ppvObj = nullptr;

    if (IsEqualIID(riid, IID_IUnknown) ||
        IsEqualIID(riid, IID_ITfTextInputProcessor)) {
        *ppvObj = static_cast<ITfTextInputProcessor*>(this);
    } else if (IsEqualIID(riid, IID_ITfTextInputProcessorEx)) {
        *ppvObj = static_cast<ITfTextInputProcessorEx*>(this);
    } else if (IsEqualIID(riid, IID_ITfThreadMgrEventSink)) {
        *ppvObj = static_cast<ITfThreadMgrEventSink*>(this);
    } else if (IsEqualIID(riid, IID_ITfKeyEventSink)) {
        *ppvObj = static_cast<ITfKeyEventSink*>(this);
    } else if (IsEqualIID(riid, IID_ITfCompositionSink)) {
        *ppvObj = static_cast<ITfCompositionSink*>(this);
    } else if (IsEqualIID(riid, IID_ITfDisplayAttributeProvider)) {
        *ppvObj = static_cast<ITfDisplayAttributeProvider*>(this);
    }

    if (*ppvObj) {
        AddRef();
        return S_OK;
    }

    return E_NOINTERFACE;
}

STDMETHODIMP_(ULONG) CPaliKeyTextService::AddRef() {
    return InterlockedIncrement(&m_cRef);
}

STDMETHODIMP_(ULONG) CPaliKeyTextService::Release() {
    ULONG count = InterlockedDecrement(&m_cRef);
    if (count == 0) {
        delete this;
    }
    return count;
}

// ----------------------------------------------------------------------------
// ITfTextInputProcessor / ITfTextInputProcessorEx
// ----------------------------------------------------------------------------

STDMETHODIMP CPaliKeyTextService::Activate(ITfThreadMgr *ptim, TfClientId tid) {
    return ActivateEx(ptim, tid, 0);
}

STDMETHODIMP CPaliKeyTextService::ActivateEx(ITfThreadMgr *ptim, TfClientId tid, DWORD dwFlags) {
    if (!ptim) return E_INVALIDARG;

    m_pThreadMgr = ptim;
    m_pThreadMgr->AddRef();
    m_tfClientId = tid;
    m_dwActivateFlags = dwFlags;

    // Advise ThreadMgrEventSink
    ITfSource *pSource = nullptr;
    if (SUCCEEDED(m_pThreadMgr->QueryInterface(IID_ITfSource, (void**)&pSource))) {
        pSource->AdviseSink(IID_ITfThreadMgrEventSink, static_cast<ITfThreadMgrEventSink*>(this), &m_threadMgrEventSinkCookie);
        pSource->Release();
    }

    // Advise KeyEventSink
    ITfKeystrokeMgr *pKeystrokeMgr = nullptr;
    if (SUCCEEDED(m_pThreadMgr->QueryInterface(IID_ITfKeystrokeMgr, (void**)&pKeystrokeMgr))) {
        pKeystrokeMgr->AdviseKeyEventSink(m_tfClientId, static_cast<ITfKeyEventSink*>(this), TRUE);
        pKeystrokeMgr->Release();
    }

    // Register Display Attribute Atom with Category Manager
    ITfCategoryMgr *pCatMgr = nullptr;
    if (SUCCEEDED(CoCreateInstance(CLSID_TF_CategoryMgr, nullptr, CLSCTX_INPROC_SERVER, IID_ITfCategoryMgr, (void**)&pCatMgr))) {
        pCatMgr->RegisterGUID(GUID_PaliKeyDisplayAttribute, &m_displayAttrAtom);
        pCatMgr->Release();
    }

    // Load active settings from %APPDATA%\PaliKey\config.json
    palikey_config_load(&m_config);

    return S_OK;
}

STDMETHODIMP CPaliKeyTextService::Deactivate() {
    // Unadvise KeyEventSink
    ITfKeystrokeMgr *pKeystrokeMgr = nullptr;
    if (m_pThreadMgr && SUCCEEDED(m_pThreadMgr->QueryInterface(IID_ITfKeystrokeMgr, (void**)&pKeystrokeMgr))) {
        pKeystrokeMgr->UnadviseKeyEventSink(m_tfClientId);
        pKeystrokeMgr->Release();
    }

    // Unadvise ThreadMgrEventSink
    ITfSource *pSource = nullptr;
    if (m_pThreadMgr && SUCCEEDED(m_pThreadMgr->QueryInterface(IID_ITfSource, (void**)&pSource))) {
        if (m_threadMgrEventSinkCookie != TF_INVALID_COOKIE) {
            pSource->UnadviseSink(m_threadMgrEventSinkCookie);
            m_threadMgrEventSinkCookie = TF_INVALID_COOKIE;
        }
        pSource->Release();
    }

    // Free all context-specific engine instances and compositions
    m_contextMgr.Clear();

    if (m_pThreadMgr) {
        m_pThreadMgr->Release();
        m_pThreadMgr = nullptr;
    }
    m_tfClientId = TF_CLIENTID_NULL;

    return S_OK;
}

// ----------------------------------------------------------------------------
// ITfThreadMgrEventSink
// ----------------------------------------------------------------------------

STDMETHODIMP CPaliKeyTextService::OnInitDocumentMgr(ITfDocumentMgr *pdim) {
    (void)pdim;
    return S_OK;
}

STDMETHODIMP CPaliKeyTextService::OnUninitDocumentMgr(ITfDocumentMgr *pdim) {
    (void)pdim;
    return S_OK;
}

STDMETHODIMP CPaliKeyTextService::OnSetFocus(ITfDocumentMgr *pdimFocus, ITfDocumentMgr *pdimPrevFocus) {
    (void)pdimFocus;
    (void)pdimPrevFocus;
    return S_OK;
}

STDMETHODIMP CPaliKeyTextService::OnPushContext(ITfContext *pic) {
    (void)pic;
    return S_OK;
}

STDMETHODIMP CPaliKeyTextService::OnPopContext(ITfContext *pic) {
    if (pic) {
        m_contextMgr.Remove(pic);
    }
    return S_OK;
}

// ----------------------------------------------------------------------------
// ITfKeyEventSink
// ----------------------------------------------------------------------------

STDMETHODIMP CPaliKeyTextService::OnSetFocus(BOOL fForeground) {
    (void)fForeground;
    return S_OK;
}

STDMETHODIMP CPaliKeyTextService::OnTestKeyDown(ITfContext *pic, WPARAM wParam, LPARAM lParam, BOOL *pfEaten) {
    if (!pfEaten) return E_INVALIDARG;
    *pfEaten = FALSE;

    if (!pic) return S_OK;

    BYTE keyState[256];
    GetKeyboardState(keyState);

    bool isCtrl = (keyState[VK_CONTROL] & 0x80) != 0;
    bool isAlt = (keyState[VK_MENU] & 0x80) != 0;
    bool isWin = ((GetKeyState(VK_LWIN) & 0x80) != 0) || ((GetKeyState(VK_RWIN) & 0x80) != 0);

    // AltGr on Windows is simulated as Ctrl + Alt
    bool isAltGr = isCtrl && isAlt;

    ContextState *pState = m_contextMgr.Find(pic);
    bool inComposition = (pState && pState->pComposition != nullptr);

    // Standard application shortcuts (Ctrl+C, Ctrl+V, Alt+F, Win+...) bypass IME
    if ((isCtrl && !isAltGr) || (isAlt && !isAltGr) || isWin) {
        if (inComposition) {
            // Commit active composition before application shortcut is dispatched
            CEditSession *pSession = new CEditSession(this, pic, 0, 0, &CPaliKeyTextService::DoCommitSession);
            HRESULT hrSession = E_FAIL;
            pic->RequestEditSession(m_tfClientId, pSession, TF_ES_SYNC | TF_ES_READWRITE, &hrSession);
            pSession->Release();
        }
        *pfEaten = FALSE;
        return S_OK;
    }

    // Navigation and editing keys flush buffer and pass through to target
    switch (wParam) {
    case VK_LEFT:
    case VK_RIGHT:
    case VK_UP:
    case VK_DOWN:
    case VK_HOME:
    case VK_END:
    case VK_PRIOR:
    case VK_NEXT:
    case VK_TAB:
    case VK_RETURN:
    case VK_ESCAPE:
        if (inComposition) {
            CEditSession *pSession = new CEditSession(this, pic, 0, 0, &CPaliKeyTextService::DoCommitSession);
            HRESULT hrSession = E_FAIL;
            pic->RequestEditSession(m_tfClientId, pSession, TF_ES_SYNC | TF_ES_READWRITE, &hrSession);
            pSession->Release();
        }
        *pfEaten = FALSE;
        return S_OK;
    }

    // Backspace key
    if (wParam == VK_BACK) {
        if (inComposition || (pState && pState->inComposition)) {
            *pfEaten = TRUE;
            return S_OK;
        }
        // Let application handle normal backspace when no composition is active
        *pfEaten = FALSE;
        return S_OK;
    }

    // Check EasyUnicode AltGr mapping
    if (isAltGr) {
        WCHAR wch[4] = {0};
        int len = ToUnicode((UINT)wParam, HIWORD(lParam) & 0xFF, keyState, wch, 4, 0);
        if (len > 0) {
            bool isShift = (keyState[VK_SHIFT] & 0x80) != 0;
            if (EasyUnicodeMap(wch[0], isShift, m_config.niggahita) != nullptr) {
                *pfEaten = TRUE;
                return S_OK;
            }
        }
        *pfEaten = FALSE;
        return S_OK;
    }

    // Printable characters
    WCHAR wch[4] = {0};
    int len = ToUnicode((UINT)wParam, HIWORD(lParam) & 0xFF, keyState, wch, 4, 0);
    if (len > 0) {
        // Any printable character can participate in Pāḷi transliteration
        if (wch[0] >= 0x20) {
            *pfEaten = TRUE;
            return S_OK;
        }
    }

    *pfEaten = FALSE;
    return S_OK;
}

STDMETHODIMP CPaliKeyTextService::OnKeyDown(ITfContext *pic, WPARAM wParam, LPARAM lParam, BOOL *pfEaten) {
    if (!pfEaten) return E_INVALIDARG;
    *pfEaten = FALSE;

    if (!pic) return S_OK;

    CEditSession *pSession = new CEditSession(this, pic, wParam, lParam, &CPaliKeyTextService::DoKeyActionSession);
    HRESULT hrSession = E_FAIL;
    HRESULT hr = pic->RequestEditSession(m_tfClientId, pSession, TF_ES_SYNC | TF_ES_READWRITE, &hrSession);
    pSession->Release();

    if (SUCCEEDED(hr) && SUCCEEDED(hrSession)) {
        *pfEaten = TRUE;
    }

    return S_OK;
}

STDMETHODIMP CPaliKeyTextService::OnTestKeyUp(ITfContext *pic, WPARAM wParam, LPARAM lParam, BOOL *pfEaten) {
    (void)pic;
    (void)wParam;
    (void)lParam;
    if (pfEaten) *pfEaten = FALSE;
    return S_OK;
}

STDMETHODIMP CPaliKeyTextService::OnKeyUp(ITfContext *pic, WPARAM wParam, LPARAM lParam, BOOL *pfEaten) {
    (void)pic;
    (void)wParam;
    (void)lParam;
    if (pfEaten) *pfEaten = FALSE;
    return S_OK;
}

STDMETHODIMP CPaliKeyTextService::OnPreservedKey(ITfContext *pic, REFGUID rguid, BOOL *pfEaten) {
    (void)pic;
    (void)rguid;
    if (pfEaten) *pfEaten = FALSE;
    return S_OK;
}

// ----------------------------------------------------------------------------
// ITfCompositionSink
// ----------------------------------------------------------------------------

STDMETHODIMP CPaliKeyTextService::OnCompositionTerminated(TfEditCookie ecWrite, ITfComposition *pComposition) {
    (void)ecWrite;
    m_contextMgr.OnCompositionTerminated(pComposition);
    return S_OK;
}

// ----------------------------------------------------------------------------
// ITfDisplayAttributeProvider
// ----------------------------------------------------------------------------

STDMETHODIMP CPaliKeyTextService::EnumDisplayAttributeInfo(IEnumTfDisplayAttributeInfo **ppEnum) {
    if (!ppEnum) return E_INVALIDARG;
    *ppEnum = new CPaliKeyEnumDisplayAttributeInfo();
    return (*ppEnum) ? S_OK : E_OUTOFMEMORY;
}

STDMETHODIMP CPaliKeyTextService::GetDisplayAttributeInfo(REFGUID guid, ITfDisplayAttributeInfo **ppInfo) {
    if (!ppInfo) return E_INVALIDARG;
    *ppInfo = nullptr;

    if (IsEqualGUID(guid, GUID_PaliKeyDisplayAttribute)) {
        *ppInfo = new CPaliKeyDisplayAttributeInfo();
        return (*ppInfo) ? S_OK : E_OUTOFMEMORY;
    }

    return E_INVALIDARG;
}

// ----------------------------------------------------------------------------
// Edit Session Handlers
// ----------------------------------------------------------------------------

HRESULT CPaliKeyTextService::SetDisplayAttribute(TfEditCookie ec, ITfContext *pic, ITfRange *pRange, TfGuidAtom guidAtom) {
    if (!pic || !pRange) return E_INVALIDARG;

    ITfProperty *pProp = nullptr;
    if (SUCCEEDED(pic->GetProperty(GUID_PROP_ATTRIBUTE, &pProp))) {
        VARIANT var;
        var.vt = VT_I4;
        var.lVal = (LONG)guidAtom;
        pProp->SetValue(ec, pRange, &var);
        pProp->Release();
    }
    return S_OK;
}

HRESULT CPaliKeyTextService::ClearDisplayAttribute(TfEditCookie ec, ITfContext *pic, ITfRange *pRange) {
    if (!pic || !pRange) return E_INVALIDARG;

    ITfProperty *pProp = nullptr;
    if (SUCCEEDED(pic->GetProperty(GUID_PROP_ATTRIBUTE, &pProp))) {
        pProp->Clear(ec, pRange);
        pProp->Release();
    }
    return S_OK;
}

void CPaliKeyTextService::CommitActiveComposition(TfEditCookie ec, ContextState *pState) {
    if (!pState || !pState->pComposition) return;

    ITfRange *pRange = nullptr;
    if (SUCCEEDED(pState->pComposition->GetRange(&pRange))) {
        if (pState->pContext) {
            ClearDisplayAttribute(ec, pState->pContext, pRange);
        }
        pRange->Release();
    }

    pState->pComposition->EndComposition(ec);
    pState->pComposition->Release();
    pState->pComposition = nullptr;
    pState->inComposition = false;

    if (pState->engine) {
        palikey_reset_instance(pState->engine);
    }
}

HRESULT CPaliKeyTextService::DoCommitSession(TfEditCookie ec, ITfContext *pic, WPARAM wParam, LPARAM lParam) {
    (void)wParam;
    (void)lParam;
    ContextState *pState = m_contextMgr.Find(pic);
    if (pState) {
        CommitActiveComposition(ec, pState);
    }
    return S_OK;
}

HRESULT CPaliKeyTextService::DoKeyActionSession(TfEditCookie ec, ITfContext *pic, WPARAM wParam, LPARAM lParam) {
    if (!pic) return E_INVALIDARG;

    ContextState *pState = m_contextMgr.GetOrCreate(pic, m_config);
    if (!pState || !pState->engine) return E_FAIL;

    BYTE keyState[256];
    GetKeyboardState(keyState);

    bool isCtrl = (keyState[VK_CONTROL] & 0x80) != 0;
    bool isAlt = (keyState[VK_MENU] & 0x80) != 0;
    bool isShift = (keyState[VK_SHIFT] & 0x80) != 0;
    bool isAltGr = isCtrl && isAlt;

    // 1. EasyUnicode AltGr shortcut handling
    if (isAltGr) {
        WCHAR wch[4] = {0};
        int len = ToUnicode((UINT)wParam, HIWORD(lParam) & 0xFF, keyState, wch, 4, 0);
        if (len > 0) {
            const WCHAR *easy = EasyUnicodeMap(wch[0], isShift, m_config.niggahita);
            if (easy) {
                CommitActiveComposition(ec, pState);

                ITfInsertAtSelection *pInsert = nullptr;
                if (SUCCEEDED(pic->QueryInterface(IID_ITfInsertAtSelection, (void**)&pInsert))) {
                    ITfRange *pRange = nullptr;
                    pInsert->InsertTextAtSelection(ec, 0, easy, (LONG)wcslen(easy), &pRange);
                    if (pRange) pRange->Release();
                    pInsert->Release();
                }
                return S_OK;
            }
        }
    }

    // 2. Backspace key handling
    if (wParam == VK_BACK) {
        PalikeyProcessResult result = {0};
        palikey_process_instance(pState->engine, 0, true, &result);

        if (result.commit[0] != '\0') {
            // Single-level backspace undo triggered! (e.g. 'ā' -> 'aa', 'ṭ' -> '.t')
            WCHAR wszCommit[64] = {0};
            MultiByteToWideChar(CP_UTF8, 0, result.commit, -1, wszCommit, 64);

            if (pState->pComposition) {
                ITfRange *pRange = nullptr;
                if (SUCCEEDED(pState->pComposition->GetRange(&pRange))) {
                    pRange->SetText(ec, 0, wszCommit, (LONG)wcslen(wszCommit));
                    ClearDisplayAttribute(ec, pic, pRange);
                    pRange->Release();
                }
                pState->pComposition->EndComposition(ec);
                pState->pComposition->Release();
                pState->pComposition = nullptr;
                pState->inComposition = false;
            } else {
                TF_SELECTION sel;
                ULONG cFetched = 0;
                if (SUCCEEDED(pic->GetSelection(ec, TF_DEFAULT_SELECTION, 1, &sel, &cFetched)) && cFetched > 0) {
                    LONG cchShifted = 0;
                    sel.range->ShiftStart(ec, -1, &cchShifted, nullptr);
                    sel.range->SetText(ec, 0, wszCommit, (LONG)wcslen(wszCommit));
                    sel.range->Collapse(ec, TF_ANCHOR_END);
                    pic->SetSelection(ec, 1, &sel);
                    sel.range->Release();
                }
            }
        } else if (pState->pComposition) {
            // Erase within in-progress composition
            CommitActiveComposition(ec, pState);
        }
        return S_OK;
    }

    // 3. Printable character handling
    WCHAR wch[4] = {0};
    int len = ToUnicode((UINT)wParam, HIWORD(lParam) & 0xFF, keyState, wch, 4, 0);
    if (len <= 0) return S_OK;

    PalikeyProcessResult result = {0};
    palikey_process_instance(pState->engine, (uint32_t)wch[0], false, &result);

    WCHAR wszCommit[64] = {0};
    if (result.commit[0] != '\0') {
        MultiByteToWideChar(CP_UTF8, 0, result.commit, -1, wszCommit, 64);
    }

    if (result.backspaces > 0) {
        // Transliteration match completed (e.g. "aa" -> "ā", ".t" -> "ṭ")
        if (pState->pComposition) {
            ITfRange *pRange = nullptr;
            if (SUCCEEDED(pState->pComposition->GetRange(&pRange))) {
                pRange->SetText(ec, 0, wszCommit, (LONG)wcslen(wszCommit));
                ClearDisplayAttribute(ec, pic, pRange);
                pRange->Release();
            }
            pState->pComposition->EndComposition(ec);
            pState->pComposition->Release();
            pState->pComposition = nullptr;
            pState->inComposition = false;
        } else {
            TF_SELECTION sel;
            ULONG cFetched = 0;
            if (SUCCEEDED(pic->GetSelection(ec, TF_DEFAULT_SELECTION, 1, &sel, &cFetched)) && cFetched > 0) {
                LONG cchShifted = 0;
                sel.range->ShiftStart(ec, -((LONG)result.backspaces), &cchShifted, nullptr);
                sel.range->SetText(ec, 0, wszCommit, (LONG)wcslen(wszCommit));
                sel.range->Collapse(ec, TF_ANCHOR_END);
                pic->SetSelection(ec, 1, &sel);
                sel.range->Release();
            }
        }
    } else if (result.commit[0] != '\0') {
        bool isWordBoundary = (wch[0] == L' ' || wch[0] == L'\t' || wch[0] == L'\r' || wch[0] == L'\n' ||
                               (wch[0] >= L'0' && wch[0] <= L'9') ||
                               wch[0] == L',' || wch[0] == L';' || wch[0] == L':' || wch[0] == L'/' ||
                               wch[0] == L'(' || wch[0] == L')' || wch[0] == L'[' || wch[0] == L']' ||
                               wch[0] == L'{' || wch[0] == L'}');

        if (isWordBoundary) {
            CommitActiveComposition(ec, pState);

            ITfInsertAtSelection *pInsert = nullptr;
            if (SUCCEEDED(pic->QueryInterface(IID_ITfInsertAtSelection, (void**)&pInsert))) {
                ITfRange *pRange = nullptr;
                pInsert->InsertTextAtSelection(ec, 0, wszCommit, (LONG)wcslen(wszCommit), &pRange);
                if (pRange) pRange->Release();
                pInsert->Release();
            }
        } else {
            // Active word typing: display composition with preedit underline
            if (!pState->pComposition) {
                ITfContextComposition *pContextComp = nullptr;
                if (SUCCEEDED(pic->QueryInterface(IID_ITfContextComposition, (void**)&pContextComp))) {
                    TF_SELECTION sel;
                    ULONG cFetched = 0;
                    if (SUCCEEDED(pic->GetSelection(ec, TF_DEFAULT_SELECTION, 1, &sel, &cFetched)) && cFetched > 0) {
                        pContextComp->StartComposition(ec, sel.range, static_cast<ITfCompositionSink*>(this), &pState->pComposition);
                        sel.range->Release();
                    }
                    pContextComp->Release();
                }
            }

            if (pState->pComposition) {
                ITfRange *pRange = nullptr;
                if (SUCCEEDED(pState->pComposition->GetRange(&pRange))) {
                    pRange->Collapse(ec, TF_ANCHOR_END);
                    pRange->SetText(ec, 0, wszCommit, (LONG)wcslen(wszCommit));

                    ITfRange *pFullRange = nullptr;
                    if (SUCCEEDED(pState->pComposition->GetRange(&pFullRange))) {
                        SetDisplayAttribute(ec, pic, pFullRange, m_displayAttrAtom);
                        pFullRange->Collapse(ec, TF_ANCHOR_END);
                        TF_SELECTION sel = { pFullRange, { static_cast<TfActiveSelEnd>(TF_AE_NONE), FALSE } };
                        pic->SetSelection(ec, 1, &sel);
                        pFullRange->Release();
                    }
                    pRange->Release();
                }
                pState->inComposition = true;
            } else {
                ITfInsertAtSelection *pInsert = nullptr;
                if (SUCCEEDED(pic->QueryInterface(IID_ITfInsertAtSelection, (void**)&pInsert))) {
                    ITfRange *pRange = nullptr;
                    pInsert->InsertTextAtSelection(ec, 0, wszCommit, (LONG)wcslen(wszCommit), &pRange);
                    if (pRange) pRange->Release();
                    pInsert->Release();
                }
            }
        }
    }

    return S_OK;
}
