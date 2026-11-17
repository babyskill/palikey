#pragma once

#ifndef PALIKEY_TSF_EDIT_SESSION_H
#define PALIKEY_TSF_EDIT_SESSION_H

#include <windows.h>
#include <msctf.h>

class CPaliKeyTextService;

class CEditSession : public ITfEditSession {
public:
    typedef HRESULT (CPaliKeyTextService::*EditSessionAction)(TfEditCookie ec, ITfContext *pic, WPARAM wParam, LPARAM lParam);

    CEditSession(CPaliKeyTextService *pTextService, ITfContext *pic, WPARAM wParam, LPARAM lParam, EditSessionAction pfnAction);
    virtual ~CEditSession();

    // IUnknown
    STDMETHODIMP QueryInterface(REFIID riid, void **ppvObj) override;
    STDMETHODIMP_(ULONG) AddRef() override;
    STDMETHODIMP_(ULONG) Release() override;

    // ITfEditSession
    STDMETHODIMP DoEditSession(TfEditCookie ec) override;

private:
    LONG m_cRef;
    CPaliKeyTextService *m_pTextService;
    ITfContext *m_pContext;
    WPARAM m_wParam;
    LPARAM m_lParam;
    EditSessionAction m_pfnAction;
};

#endif // PALIKEY_TSF_EDIT_SESSION_H
