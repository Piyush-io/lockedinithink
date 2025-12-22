#include <cerrno>
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <functional>
#include <iostream>
#include <string>
#include <sys/wait.h>
#include <unistd.h>
#include <unordered_map>
#include <utility>
#include <vector>
using namespace std;

const int buf_size = 1024;
string mush_readline(string &input) {
  input.reserve(buf_size);
  int c;

  while (true) {
    c = cin.get();
    if (c == '\n' || c == EOF)
      break;
    input.push_back(static_cast<char>(c));
  }

  return input;
}

vector<string> mush_readargs(string &input) {
  vector<string> args;
  string arg;
  for (char x : input) {
    if (x == ' ' || x == '\t') {
      if (!arg.empty()) {
        args.push_back(std::move(arg));
        arg.clear();
      }
    } else if (x == '\n')
      break;
    else
      arg += x;
  }
  if (!arg.empty())
    args.push_back(std::move(arg));
  return args;
}

int mush_cd(vector<string> args);
int mush_help(vector<string> args);
int mush_exit(vector<string> args);

const unordered_map<string, function<int(vector<string>)>> builtins = {
    {"cd", mush_cd}, {"exit", mush_exit}, {"help", mush_help}};

int mush_cd(vector<string> args) {
  if (args[1].empty() || args[1].size() < 2)
    cerr << " mush: expected argument to \"cd\"\n";
  else {
    if (chdir(args[1].c_str()) != 0) {
      perror("mush");
    }
  }
  return 1;
}

int mush_exit(vector<string> args) { return 0; }

int mush_help(vector<string> args) {
  int i;
  cout << "Piyush says sssshhh..." << endl;
  cout << "The following commands are built in for you to use: ";
  for (auto &x : builtins) {
    cout << x.first << endl;
  }
  return 1;
}

int mush_launch(vector<string> args) {
  pid_t pid, wpid;
  int status;

  pid = fork();
  if (pid == 0) {
    vector<const char *> argv;
    for (const auto &arg : args) {
      argv.push_back(arg.c_str());
    }
    argv.push_back(nullptr);
    if (execvp(argv[0], (char *const*)argv.data()) == -1) {
      perror("mush");
    }
    exit(EXIT_FAILURE);
  } else if (pid < 0) {
    perror("mush");
  } else {
    do {
      wpid = waitpid(pid, &status, WUNTRACED);
    } while (!WIFEXITED(status) && !WIFSIGNALED(status));
  }
  return 1;
}

int mush_execute(const vector<string> args) {
  for (auto &x : builtins) {
    if (args[0] == x.first) {
      return x.second(args);
    }
  }
  return mush_launch(args);
}

void mush_loop() {
  string input_line;
  vector<string> args;
  int status;

  do {
    cout << ">";
    input_line = mush_readline(input_line);
    args = mush_readargs(input_line);
    status = mush_execute(args);

    input_line.clear();
    args.clear();
  } while (status);
}

int main() {
  mush_loop();
  return 0;
}
