#include "edit_session.h"
#include "palikey_tsf.h"

CEditSession::CEditSession(CPaliKeyTextService *pTextService, ITfContext *pic, WPARAM wParam, LPARAM lParam, EditSessionAction pfnAction)
    : m_cRef(1), m_pTextService(pTextService), m_pContext(pic), m_wParam(wParam), m_lParam(lParam), m_pfnAction(pfnAction) {
    if (m_pContext) {
        m_pContext->AddRef();
    }
}

CEditSession::~CEditSession() {
    if (m_pContext) {
        m_pContext->Release();
    }
}

STDMETHODIMP CEditSession::QueryInterface(REFIID riid, void **ppvObj) {
    if (!ppvObj) return E_INVALIDARG;
    *ppvObj = nullptr;

    if (IsEqualIID(riid, IID_IUnknown) || IsEqualIID(riid, IID_ITfEditSession)) {
        *ppvObj = static_cast<ITfEditSession*>(this);
        AddRef();
        return S_OK;
    }
    return E_NOINTERFACE;
}

STDMETHODIMP_(ULONG) CEditSession::AddRef() {
    return InterlockedIncrement(&m_cRef);
}

STDMETHODIMP_(ULONG) CEditSession::Release() {
    ULONG count = InterlockedDecrement(&m_cRef);
    if (count == 0) {
        delete this;
    }
    return count;
}

STDMETHODIMP CEditSession::DoEditSession(TfEditCookie ec) {
    if (m_pTextService && m_pfnAction && m_pContext) {
        return (m_pTextService->*m_pfnAction)(ec, m_pContext, m_wParam, m_lParam);
    }
    return E_FAIL;
}
