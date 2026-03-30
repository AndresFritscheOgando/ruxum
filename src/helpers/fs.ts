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

export async function copyTemplate(
  templatePath: string,
  targetPath: string,
  variables: Record<string, string> = {}
): Promise<void> {
  const templateDir = path.resolve(__dirname, "../../template");
  const fullTemplatePath = path.join(templateDir, templatePath);
  
  let content = await fse.readFile(fullTemplatePath, "utf-8");
  
  for (const [key, value] of Object.entries(variables)) {
    content = content.replace(new RegExp(`{{${key}}}`, "g"), value);
  }
  
  await fse.ensureDir(path.dirname(targetPath));
  await fse.writeFile(targetPath, content, "utf-8");
}
