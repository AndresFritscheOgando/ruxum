import * as fse from "fs-extra";
import * as path from "path";

export async function ensureDir(dirPath: string): Promise<void> {
  await fse.ensureDir(dirPath);
}

export async function writeProjectFile(
  projectDir: string,
  relativePath: string,
  content: string
): Promise<void> {
  const fullPath = path.join(projectDir, relativePath);
  await fse.ensureDir(path.dirname(fullPath));
  await fse.writeFile(fullPath, content, "utf-8");
}
