const fs = require('fs');
const current_version = JSON.parse(fs.readFileSync('package.json')).version
// 定义文件路径数组和要替换的文本
const filePaths = [
  'src-tauri/tauri.conf.json',
  'src-tauri/Cargo.toml',
  'package.json',
  'src/App.vue'
];
const next_version = updateVersion(current_version);
console.log("Next version: ", next_version)
// 遍历文件路径数组
const tj = JSON.parse(fs.readFileSync(filePaths[0]))
tj.package.version = next_version
fs.writeFileSync(filePaths[0], JSON.stringify(tj,{},'  '), { encoding: 'utf-8' })

let tm = fs.readFileSync(filePaths[1]).toString('utf-8')
tm = tm.replace(`[package]
version = "${current_version}"`, `[package]
version = "${next_version}"`)
fs.writeFileSync(filePaths[1], tm, { encoding: 'utf-8' })

const pj = JSON.parse(fs.readFileSync(filePaths[2]))
pj.version = next_version
fs.writeFileSync(filePaths[2], JSON.stringify(pj,{},'  '), { encoding: 'utf-8' })

let ae = fs.readFileSync(filePaths[3]).toString('utf-8')
ae = ae.replace(`文件 MD5 校对器 v${current_version}`, `文件 MD5 校对器 v${next_version}`)
fs.writeFileSync(filePaths[3], ae, { encoding: 'utf-8' })

function updateVersion(currentVersion) {
  // 将版本号拆分成数组
  const versionArray = currentVersion.split('.').map(Number);
  const majorUpdate = parseInt(process.argv[2] ?? 1)
  switch (majorUpdate) {
    case 3:
      versionArray[0]++;
      versionArray[1] = 0;
      versionArray[2] = 0;
      break
    case 2:
      versionArray[1]++;
      versionArray[2] = 0;
      break
    case 1:
      versionArray[2]++;
      break
    case -1:
      versionArray[2] -= 1;
      break
  }
  return versionArray.join('.');
}
