#include <stdio.h>
#include <iostream>
#include <string>

using namespace std;
int main() {
string cmd;
cout << "==tp==summon==" << endl;
getline(cin, cmd);

if (cmd == "tp") {
   string who;
   cout << "podaj kogo, (twoj nick), @a, @e, @p, @n" << endl;
   getline(cin, who);
   string towhocords;
   cout << "do kogo albo kordy" << endl;
   getline(cin,towhocords);
   cout << "twoja komenda to:" << endl;
   cout << "/tp " << who << "" << towhocords << endl;
   cin.get();


}
if (cmd == "summon") {
  string mob;
  cout << "podaj jaki mob np bee albo blaze" << endl;
  getline(cin, mob);
  cout << "komenda to: " << endl;
  cout << "/summon minecraft:" "" << mob << endl;
  cin.get();

}

}