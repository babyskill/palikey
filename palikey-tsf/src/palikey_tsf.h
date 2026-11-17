#pragma once

#ifndef PALIKEY_TSF_H
#define PALIKEY_TSF_H

#include <windows.h>
#include <msctf.h>
#include "context_state.h"
#include "palikey_config.h"

class CPaliKeyTextService :
    public ITfTextInputProcessorEx,
    public ITfThreadMgrEventSink,
    public ITfKeyEventSink,
    public ITfCompositionSink,
    public ITfDisplayAttributeProvider {

public:
    CPaliKeyTextService();
    virtual ~CPaliKeyTextService();

    // IUnknown
    STDMETHODIMP QueryInterface(REFIID riid, void **ppvObj) override;
    STDMETHODIMP_(ULONG) AddRef() override;
    STDMETHODIMP_(ULONG) Release() override;

    // ITfTextInputProcessor
    STDMETHODIMP Activate(ITfThreadMgr *ptim, TfClientId tid) override;
    STDMETHODIMP Deactivate() override;

    // ITfTextInputProcessorEx
    STDMETHODIMP ActivateEx(ITfThreadMgr *ptim, TfClientId tid, DWORD dwFlags) override;

    // ITfThreadMgrEventSink
    STDMETHODIMP OnInitDocumentMgr(ITfDocumentMgr *pdim) override;
    STDMETHODIMP OnUninitDocumentMgr(ITfDocumentMgr *pdim) override;
    STDMETHODIMP OnSetFocus(ITfDocumentMgr *pdimFocus, ITfDocumentMgr *pdimPrevFocus) override;
    STDMETHODIMP OnPushContext(ITfContext *pic) override;
    STDMETHODIMP OnPopContext(ITfContext *pic) override;

    // ITfKeyEventSink
    STDMETHODIMP OnSetFocus(BOOL fForeground) override;
    STDMETHODIMP OnTestKeyDown(ITfContext *pic, WPARAM wParam, LPARAM lParam, BOOL *pfEaten) override;
    STDMETHODIMP OnKeyDown(ITfContext *pic, WPARAM wParam, LPARAM lParam, BOOL *pfEaten) override;
    STDMETHODIMP OnTestKeyUp(ITfContext *pic, WPARAM wParam, LPARAM lParam, BOOL *pfEaten) override;
    STDMETHODIMP OnKeyUp(ITfContext *pic, WPARAM wParam, LPARAM lParam, BOOL *pfEaten) override;
    STDMETHODIMP OnPreservedKey(ITfContext *pic, REFGUID rguid, BOOL *pfEaten) override;

    // ITfCompositionSink
    STDMETHODIMP OnCompositionTerminated(TfEditCookie ecWrite, ITfComposition *pComposition) override;

    // ITfDisplayAttributeProvider
    STDMETHODIMP EnumDisplayAttributeInfo(IEnumTfDisplayAttributeInfo **ppEnum) override;
    STDMETHODIMP GetDisplayAttributeInfo(REFGUID guid, ITfDisplayAttributeInfo **ppInfo) override;

    // Edit session handlers called by CEditSession
    HRESULT DoKeyActionSession(TfEditCookie ec, ITfContext *pic, WPARAM wParam, LPARAM lParam);
    HRESULT DoCommitSession(TfEditCookie ec, ITfContext *pic, WPARAM wParam, LPARAM lParam);

private:
    LONG m_cRef;
    ITfThreadMgr *m_pThreadMgr;
    TfClientId m_tfClientId;
    DWORD m_dwActivateFlags;
    DWORD m_threadMgrEventSinkCookie;
    TfGuidAtom m_displayAttrAtom;

    ContextManager m_contextMgr;
    PalikeyConfig m_config;

    HRESULT SetDisplayAttribute(TfEditCookie ec, ITfContext *pic, ITfRange *pRange, TfGuidAtom guidAtom);
    HRESULT ClearDisplayAttribute(TfEditCookie ec, ITfContext *pic, ITfRange *pRange);
    void CommitActiveComposition(TfEditCookie ec, ContextState *pState);
};

#endif // PALIKEY_TSF_H
