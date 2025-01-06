# Rust Course Assignment Submission Guide 📚🦀

Welcome to the Rust Course! This guide will help you submit your assignments step-by-step using Git and GitHub. Follow these instructions carefully to ensure your work is submitted correctly.

---

## Step 1: Fork the Repository 🍴
1. Go to the course repository: [ibilalkayy/course](https://github.com/ibilalkayy/course).
2. Click the **Fork** button in the top-right corner to create a copy of the repository under your GitHub account.

---

## Step 2: Clone the Forked Repository 💻
1. Open your terminal or Git Bash.
2. Clone your forked repository to your local machine:
   ```bash
   git clone https://github.com/<your-username>/course.git
   ```
   Replace `<your-username>` with your GitHub username.

3. Navigate to the project folder:
   ```bash
   cd course
   ```

---

## Step 3: Create a New Branch 🌱
1. Create a new branch for your assignment:
   ```bash
   git checkout -b assignment-number-<your-name>
   ```
   Replace `<your-name>` with your name or the assignment title.

---

## Step 4: Add Your Assignment 📝
1. Add your assignment files to the appropriate folder.
2. If there is no specific folder for your assignment, create one with your name and put the assignment file or the code in it.

---

## Step 5: Commit Your Changes ✅
1. Stage your changes:
   ```bash
   git add .
   ```
2. Commit your changes with a meaningful message:
   ```bash
   git commit -m "Add assignment for Rust course by <your-name>"
   ```

---

## Step 6: Push Your Branch to GitHub 🚀
1. Go to the main branch again from this command:
  ```bash
   git checkout main
   ```
2. Push your branch to your forked repository:
   ```bash
   git push origin assignment-number-<your-name>
   ```

---

## Step 7: Create a Pull Request (PR) 🔁
1. Go to the original course repository: [ibilalkayy/course](https://github.com/ibilalkayy/course).
2. Click the **Pull Requests** tab.
3. Click the **New Pull Request** button.
4. Select your branch (`assignment-number-<your-name>`) from your forked repository as the source branch.
5. Add a meaningful title and description to your PR.
6. Click **Create Pull Request**.

---

## Step 8: Wait for Feedback 👨‍🏫
1. Your instructor will review your submission and may provide feedback or request changes.
2. If requested, make the changes and update your PR:
   - Edit your files locally.
   - Commit and push the changes:
     ```bash
     git add .
     git commit -m "Update assignment as per feedback"
     git push origin assignment-number-<your-name>
     ```

---

### Notes
- **Deadline**: Ensure you submit your assignment before the deadline.
- **Naming Convention**: Follow the file naming conventions provided by the instructor.
- **Help**: If you face any issues, feel free to ask for help on the Discord server.

---

Happy coding, and good luck with your Rust journey! 🚀🦀
