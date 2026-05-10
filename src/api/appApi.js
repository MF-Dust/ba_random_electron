import { invoke, listenCompat } from './tauriCore'

export const appApi = {
  getConfig: () => invoke('get_config'),
  parseStudentListText: (rawText, existingStudents) => invoke('parse_student_list_text', { rawText, existingStudents }),
  importStudentListFromFile: (existingStudents) => invoke('import_student_list_from_file', { existingStudents }),
  saveConfig: (config) => invoke('save_app_config', { config }),
  getAppInfo: () => invoke('get_app_info'),
  checkUpdate: () => invoke('check_update'),
  requestAdminElevation: () => invoke('request_admin_elevation'),
  createAdminStartupTask: (exePath, taskName) => invoke('create_admin_startup_task', { exePath, taskName }),
  getLogs: () => invoke('get_logs'),
  onLogEntry: (callback) => listenCompat('log-entry', callback)
}
