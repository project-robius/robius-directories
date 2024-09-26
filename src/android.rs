extern crate dirs_sys;

use std::{env, path::PathBuf};

use jni::{
    objects::{JObject, JString, JValue, JValueGen},
    JNIEnv,
};
use BaseDirs;
use ProjectDirs;
use UserDirs;

pub fn base_dirs() -> Option<BaseDirs> {
    todo!()
}

pub fn user_dirs() -> Option<UserDirs> {
    todo!()
}

pub fn project_dirs_from_path(project_path: PathBuf) -> Option<ProjectDirs> {
    robius_android_env::with_activity(|env, context| {
        const GET_DIR_SIG: &str = "(Ljava/lang/String;I)Ljava/io/File;";
        const GET_X_DIR_SIG: &str = "()Ljava/io/File;";

        let cache_dir = get_path_from_fn(env, context, "getCacheDir", GET_X_DIR_SIG, &[])?;

        let config_dir = get_path_from_fn(env, context, "getFilesDir", GET_X_DIR_SIG, &[])?;
        let config_local_dir = config_dir.clone();
        let data_dir = config_dir.clone();
        let data_local_dir = config_dir.clone();

        const MODE_PRIVATE: i32 = 0;
        let preference_dir = get_path_from_fn(
            env,
            context,
            "getDir",
            GET_DIR_SIG,
            &[
                JValueGen::Object(&env.new_string("shared_prefs").ok()?.into()),
                // We only need to call `getAbsolutePath` on the file object so
                // opening in private mode is sufficient.
                JValueGen::Int(MODE_PRIVATE),
            ],
        )?;

        Some(ProjectDirs {
            project_path: project_path.clone(),
            cache_dir,
            config_dir,
            config_local_dir,
            data_dir,
            data_local_dir,
            preference_dir,
            runtime_dir: None,
            state_dir: None,
        })
    })
    .flatten()
}

fn get_path_from_fn(
    env: &mut JNIEnv,
    context: &JObject,
    fn_name: &str,
    signature: &str,
    args: &[JValue],
) -> Option<PathBuf> {
    let file = env
        .call_method(context, fn_name, signature, args)
        .ok()?
        .l()
        .ok()?;
    let jobject = env
        .call_method(file, "getAbsolutePath", "()Ljava/lang/String;", &[])
        .ok()?
        .l()
        .ok()?;
    let jstring = JString::from(jobject);
    let java_str = env.get_string(&jstring).ok()?;
    Some(PathBuf::from(java_str.to_str().ok()?))
}

pub fn project_dirs_from(
    _qualifier: &str,
    _organization: &str,
    _application: &str,
) -> Option<ProjectDirs> {
    ProjectDirs::from_path(PathBuf::new())
}
