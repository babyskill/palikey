/**
 * @file native-lib.cpp
 * @brief JNI bridge between Kotlin NativeEngine and Rust palikey-engine via C ABI.
 *
 * Links libpalikey_engine.a statically and exposes JNI methods for
 * session-based Pali transliteration on Android.
 */

#include <jni.h>
#include <string>
#include <cstring>
#include <android/log.h>
#include "palikey.h"

#define LOG_TAG "PaliKeyJNI"
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO, LOG_TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)

extern "C" {

/* ========================================================================= */
/* Companion / Static Methods                                                */
/* ========================================================================= */

JNIEXPORT jstring JNICALL
Java_org_giacngo_palikey_NativeEngine_00024Companion_nativeVersion(
    JNIEnv *env,
    jobject /* this */
) {
    const char *ver = palikey_version();
    return env->NewStringUTF(ver ? ver : "0.1.0");
}

JNIEXPORT jstring JNICALL
Java_org_giacngo_palikey_NativeEngine_nativeVersion(
    JNIEnv *env,
    jclass /* clazz */
) {
    const char *ver = palikey_version();
    return env->NewStringUTF(ver ? ver : "0.1.0");
}

/* ========================================================================= */
/* Instance Lifecycle Methods                                                */
/* ========================================================================= */

JNIEXPORT jlong JNICALL
Java_org_giacngo_palikey_NativeEngine_nativeCreate(
    JNIEnv * /* env */,
    jobject /* this */,
    jint scheme_id,
    jint niggahita,
    jboolean extended_set
) {
    PalikeyEngine *engine = palikey_new(
        static_cast<uint32_t>(scheme_id),
        static_cast<uint32_t>(niggahita),
        extended_set == JNI_TRUE
    );
    if (!engine) {
        LOGE("Failed to allocate PalikeyEngine instance");
        return 0;
    }
    return reinterpret_cast<jlong>(engine);
}

JNIEXPORT void JNICALL
Java_org_giacngo_palikey_NativeEngine_nativeDestroy(
    JNIEnv * /* env */,
    jobject /* this */,
    jlong handle
) {
    PalikeyEngine *engine = reinterpret_cast<PalikeyEngine *>(handle);
    if (engine) {
        palikey_free(engine);
    }
}

/* ========================================================================= */
/* Keystroke Processing & State Methods                                      */
/* ========================================================================= */

JNIEXPORT jobject JNICALL
Java_org_giacngo_palikey_NativeEngine_nativeProcess(
    JNIEnv *env,
    jobject /* this */,
    jlong handle,
    jint key_char,
    jboolean is_backspace
) {
    PalikeyEngine *engine = reinterpret_cast<PalikeyEngine *>(handle);
    PalikeyProcessResult result;
    std::memset(&result, 0, sizeof(result));

    if (engine) {
        palikey_process_instance(
            engine,
            static_cast<uint32_t>(key_char),
            is_backspace == JNI_TRUE,
            &result
        );
    } else {
        LOGE("nativeProcess called with null engine handle");
    }

    jclass resultClass = env->FindClass("org/giacngo/palikey/ProcessResult");
    if (!resultClass) {
        LOGE("Cannot find class org/giacngo/palikey/ProcessResult");
        return nullptr;
    }

    jmethodID ctor = env->GetMethodID(
        resultClass,
        "<init>",
        "(ILjava/lang/String;Ljava/lang/String;)V"
    );
    if (!ctor) {
        LOGE("Cannot find constructor for ProcessResult");
        env->DeleteLocalRef(resultClass);
        return nullptr;
    }

    jstring commitStr = env->NewStringUTF(result.commit);
    jstring preeditStr = env->NewStringUTF(result.preedit);

    jobject resObj = env->NewObject(
        resultClass,
        ctor,
        static_cast<jint>(result.backspaces),
        commitStr,
        preeditStr
    );

    env->DeleteLocalRef(commitStr);
    env->DeleteLocalRef(preeditStr);
    env->DeleteLocalRef(resultClass);

    return resObj;
}

JNIEXPORT jboolean JNICALL
Java_org_giacngo_palikey_NativeEngine_nativeSetScheme(
    JNIEnv * /* env */,
    jobject /* this */,
    jlong handle,
    jint scheme_id
) {
    PalikeyEngine *engine = reinterpret_cast<PalikeyEngine *>(handle);
    if (!engine) return JNI_FALSE;

    int32_t ok = palikey_set_scheme(engine, static_cast<uint32_t>(scheme_id));
    return ok ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT void JNICALL
Java_org_giacngo_palikey_NativeEngine_nativeReset(
    JNIEnv * /* env */,
    jobject /* this */,
    jlong handle
) {
    PalikeyEngine *engine = reinterpret_cast<PalikeyEngine *>(handle);
    if (engine) {
        palikey_reset_instance(engine);
    }
}

/* ========================================================================= */
/* JNI Dynamic Registration (JNI_OnLoad)                                     */
/* ========================================================================= */

static const JNINativeMethod gNativeEngineMethods[] = {
    {
        (char *)"nativeCreate",
        (char *)"(IIZ)J",
        (void *)Java_org_giacngo_palikey_NativeEngine_nativeCreate
    },
    {
        (char *)"nativeDestroy",
        (char *)"(J)V",
        (void *)Java_org_giacngo_palikey_NativeEngine_nativeDestroy
    },
    {
        (char *)"nativeProcess",
        (char *)"(JIZ)Lorg/giacngo/palikey/ProcessResult;",
        (void *)Java_org_giacngo_palikey_NativeEngine_nativeProcess
    },
    {
        (char *)"nativeSetScheme",
        (char *)"(JI)Z",
        (void *)Java_org_giacngo_palikey_NativeEngine_nativeSetScheme
    },
    {
        (char *)"nativeReset",
        (char *)"(J)V",
        (void *)Java_org_giacngo_palikey_NativeEngine_nativeReset
    }
};

static const JNINativeMethod gNativeEngineCompanionMethods[] = {
    {
        (char *)"nativeVersion",
        (char *)"()Ljava/lang/String;",
        (void *)Java_org_giacngo_palikey_NativeEngine_00024Companion_nativeVersion
    }
};

JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM *vm, void * /* reserved */) {
    JNIEnv *env = nullptr;
    if (vm->GetEnv(reinterpret_cast<void **>(&env), JNI_VERSION_1_6) != JNI_OK) {
        LOGE("JNI_OnLoad: GetEnv failed");
        return JNI_ERR;
    }

    jclass engineClass = env->FindClass("org/giacngo/palikey/NativeEngine");
    if (engineClass) {
        env->RegisterNatives(
            engineClass,
            gNativeEngineMethods,
            sizeof(gNativeEngineMethods) / sizeof(gNativeEngineMethods[0])
        );
        env->DeleteLocalRef(engineClass);
    }

    jclass companionClass = env->FindClass("org/giacngo/palikey/NativeEngine$Companion");
    if (companionClass) {
        env->RegisterNatives(
            companionClass,
            gNativeEngineCompanionMethods,
            sizeof(gNativeEngineCompanionMethods) / sizeof(gNativeEngineCompanionMethods[0])
        );
        env->DeleteLocalRef(companionClass);
    }

    LOGI("JNI_OnLoad successfully registered native methods");
    return JNI_VERSION_1_6;
}

} // extern "C"
