# Save Backup

A super simple and low resource automatic backup maker.

## Configuration

Save Backup uses a YAML file named `config.yaml` in the working directory to configure what the application does.

Common default options consist of:

| Key         | Type      | Meaning                                                      | Default          |
| ----------- | --------- | ------------------------------------------------------------ | ---------------- |
| `save_root` | `string`  | The root folder where backups are stored                     | `./save-backups` |
| `zip`       | `boolean` | Whether to compress backups as zip files                     | `true`           |
| `count`     | `integer` | Number of backups per name before oldest backups get removed | `10`             |
| `interval`  | `integer` | Number of minutes between each backup                        | `30`             |

Option fields specific to each `name` consist of:

| Key       | Type       | Meaning                                                                                                                          |
| --------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `root`    | `string`   | Root directory of where save files are found                                                                                     |
| `include` | `string[]` | Array of files (can't be directories) relative to `root` to include in backup. If not provided, all files in `root` are included |
| `exclude` | `string[]` | Array of 
