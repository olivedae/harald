#ifndef bluetooth_osx_h
#define bluetooth_osx_h

NSArray * bluetooth_scan(void);

@class IOBluetoothDeviceInquiry;

@interface Scanner : NSObject
{
    IOBluetoothDeviceInquiry *inquiry;
    BOOL busy;
    NSMutableArray *foundDevices;
}

-(IOReturn) startSearch;
-(void)     stopSearch;
-(BOOL)     isBusy;
-(NSArray *)getFoundDevices;

@end

@interface Device : NSObject

@property NSString *name, *address;

-(void) setName:(NSString *)name andAddress:(NSString *)address;

@end

#endif /* bluetooth_osx_h */
